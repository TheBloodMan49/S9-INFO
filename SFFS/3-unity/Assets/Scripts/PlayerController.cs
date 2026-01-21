using UnityEngine;
using UnityEngine.InputSystem;

public class PlayerController : MonoBehaviour
{
    private BoardManager m_Board;
    private Vector2Int m_CellPosition;
    // private Vector3 offset = new Vector3(0.0f, 0.0f, 0.0f); // Ajuste au besoin

    //private bool m_IsGameOver = false;

    public void Spawn(BoardManager boardManager, Vector2Int cell)
    {
        // TODO : stocker la r�f�rence au BoardManager
        m_Board = boardManager;
        // TODO : stocker la cellule actuelle
        m_CellPosition = cell;

        // TODO : convertir la cellule en position monde
        Vector3 worldPos = m_Board.CellToWorld(cell);
        // TODO : positionner le joueur � cette position: transform.position = ? 
        transform.position = worldPos;
    }

    public Vector2Int Cell
    {
        get { return m_CellPosition; }
    }

    private void Update()
    {
        Vector2Int newCellTarget = m_CellPosition;
        bool hasMoved = false;
        if(Keyboard.current.wKey.wasPressedThisFrame)
        {
            newCellTarget.y += 1;
            hasMoved = true;
        }
        // TODO : finir le reste des touches pour se déplacer
        if(Keyboard.current.sKey.wasPressedThisFrame)
        {
            newCellTarget.y -= 1;
            hasMoved = true;
        }
        if(Keyboard.current.aKey.wasPressedThisFrame)
        {
            newCellTarget.x -= 1;
            hasMoved = true;
        }
        if(Keyboard.current.dKey.wasPressedThisFrame)
        {
            newCellTarget.x += 1;
            hasMoved = true;
        }
        if (hasMoved)
        {
            //Vérifie que la cell est passable et y aller si c’est le cas.
            BoardManager.CellData cellData = m_Board.GetCellData(newCellTarget);
            if (cellData != null && cellData.Passable)
            {
                GameManager.Instance.TurnManager.Tick();
                if (cellData.ContainedObject == null) {
                    MoveTo(newCellTarget);
                } else if (cellData.ContainedObject.PlayerWantsToEnter()) {
                    MoveTo(newCellTarget);
                    cellData.ContainedObject.PlayerEntered();
                }
            }
        }
    }

    void MoveTo(Vector2Int cell) {
        m_CellPosition = cell;
        transform.position = m_Board.CellToWorld(cell);
    }
}
