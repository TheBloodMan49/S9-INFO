"""
Un ensemble de quelques fonctions utilitaires, pour le TP.
"""

from IPython.display import display
import sqlite3
import pandas as pd
import os
import uuid
from langchain_core.tracers.context import tracing_v2_enabled
from langchain_core.messages import HumanMessage
from rich.console import Console
from rich.panel import Panel
from rich.markdown import Markdown
from rich.live import Live
from rich.text import Text
from rich.spinner import Spinner
from rich.padding import Padding
from rich.rule import Rule
from rich.syntax import Syntax
from rich.prompt import Confirm
from langgraph.graph.state import CompiledStateGraph
from langgraph.types import Command

def display_database(path):
    """
    Affiche le contenu actuel de toutes les tables d'une base de données SQLite
    """
    assert os.path.isfile(path), "La base de données n'existe pas !"

    conn = sqlite3.connect(path)
    cursor = conn.cursor()
    
    # Liste des tables à afficher
    cursor.execute("SELECT name FROM sqlite_master WHERE type='table' AND name NOT LIKE 'sqlite_%';")
    tables = cursor.fetchall()

    print(f"\n{'='*60}")
    print(f"BASE DE DONNÉES : '{path}'")
    print(f"{'='*60}")

    for t in tables:
        table_name = t[0]

        print(f"\nTABLE : '{table_name}'")
        print(f"{'-'*60}")
        
        try:
            # Récupérer les donnéesFa
            df = pd.read_sql_query(f"SELECT * FROM {table_name}", conn)

            # Afficher
            with pd.option_context('display.max_rows', None, 'display.max_columns', None):
                display(df.style.hide(axis="index"))
                    
        except sqlite3.OperationalError as e:
            print(f"Erreur : La table '{table_name}' n'existe pas ou est inaccessible.")
            print(e)

    print(f"\n{'='*60}\n")
    conn.close()

def run_app_with_rich(app: CompiledStateGraph, langsmith_tracing=True):
    """
    Fais tourner une application langGraph dans la console avec gestion des interruptions (Human-in-the-loop).
    """
    def _run():
        # Thread_id unique pour la mémoire
        thread_id = str(uuid.uuid4())
        config = {"configurable": {"thread_id": thread_id}}
    
        console = Console()
    
        print()
        console.print(Panel.fit(
            "[bold cyan]🤖 ASSISTANT DE MAINTENANCE INDUSTRIELLE[/bold cyan]",
            border_style="cyan"
        ))
        print()
    
        while True:
            # 1. Entrée utilisateur principale
            user_input = console.input("[bold green]Humain (tapez 'q' pour quitter) > [/bold green]")
            
            if user_input.lower() in ["q", "quit", "exit"]:
                console.print("[red]Fin de la session[/red]")
                break
    
            # On prépare l'input initial. 
            # On utilise une variable `current_inputs` qui pourra être modifiée en cas d'interruption.
            current_inputs = {"messages": [HumanMessage(content=user_input)]}
    
            # 2. Boucle de traitement (gère les reprises après interruption)
            while current_inputs is not None:
                
                # Indicateur pour savoir si on doit sortir de la boucle interne
                should_continue_streaming = False

                with Live(Spinner("dots", text="[yellow]Traitement en cours...[/yellow]"), refresh_per_second=10, transient=True) as live:
                    
                    # Exécution du graphe
                    for event in app.stream(current_inputs, config=config):
                        
                        for node_name, state_update in event.items():
                            
                            # Gestion de l'affichage des messages
                            if "messages" in state_update and len(state_update["messages"]) > 0:
                                last_msg = state_update["messages"][-1]
                                
                                # CAS A : RÉPONSE FINALE (Adapté à votre logique Supervisor)
                                if node_name == "Supervisor" and state_update.get("next") == "FINISH":
                                    live.stop()
                                    md_report = Markdown(last_msg.content)
                                    console.print(Panel(
                                        md_report,
                                        title="[bold green]Réponse de l'assistant[/bold green]",
                                        border_style="green",
                                        expand=False
                                    ))
                                    print()
                                
                                # CAS B : ETAPE INTERMÉDIAIRE
                                else:
                                    sender = getattr(last_msg, "name", node_name)
                                    live.console.print(f"[bold dim]💭 {node_name} :[/bold dim]")
                                    
                                    # Petit bonus : Si c'est du SQL, on l'affiche joliment
                                    content_str = str(last_msg.content)
                                    if "SELECT" in content_str or "INSERT" in content_str:
                                        syntax = Syntax(content_str, "sql", theme="monokai", line_numbers=False)
                                        live.console.print(Padding(syntax, (0, 0, 0, 4)))
                                    else:
                                        md_content = Markdown(content_str)
                                        live.console.print(Padding(md_content, (0, 0, 0, 4), style="dim"))
                                    
                                    live.console.print(Rule(style="dim", characters=" "))

                # 3. Vérification post-stream : Est-ce qu'on s'est arrêté à cause d'un 'interrupt' ?
                snapshot = app.get_state(config)
                
                if snapshot.next:
                    # On vérifie s'il y a des tâches interrompues
                    if snapshot.tasks and snapshot.tasks[0].interrupts:
                        interrupt_value = snapshot.tasks[0].interrupts[0].value
                        
                        # --- INTERVENTION HUMAINE ---
                        console.print(f"\n[bold red]⚠️  ALERTE DE SÉCURITÉ[/bold red]")
                        console.print(f"[yellow]{interrupt_value.get('warning', 'Validation requise')}[/yellow]")
                        
                        query = interrupt_value.get('query', '')
                        if query:
                            console.print(Panel(Syntax(query, "sql", theme="monokai"), title="Requête en attente"))

                        # On utilise Rich Confirm pour une belle interaction
                        is_approved = Confirm.ask("Autoriser cette action ?")

                        if is_approved:
                            console.print("[green]Action approuvée. Reprise du graphe...[/green]")
                            # On reprend avec l'action "approve"
                            current_inputs = Command(resume={"action": "approve"})
                        else:
                            console.print("[red]Action rejetée.[/red]")
                            # On reprend avec l'action "reject"
                            current_inputs = Command(resume={"action": "reject"})
                        
                        # On signale qu'on doit refaire un tour de boucle 'while current_inputs'
                        should_continue_streaming = True
                
                # Si on n'a pas été interrompu, on termine la boucle de traitement interne
                # pour revenir à la saisie utilisateur principale
                if not should_continue_streaming:
                    current_inputs = None

    if langsmith_tracing:
        # Note: assurez-vous que tracing_v2_enabled est importé ou géré
        try:
            from langchain_core.tracers.context import tracing_v2_enabled
            with tracing_v2_enabled():
                return _run()
        except ImportError:
             return _run()
    else:
        return _run()