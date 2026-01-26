use std::fmt::{Debug, Display, Formatter};

#[derive(Debug)]
pub enum Error {
    FileUnreadable,
    FileUnwritable,
    CorruptedPng,
    IncorrectKey(String),
    UnknownCommand(String),
    NoMessageFound,
    DeletionFailure,
    NoCommand,
    IncorrectNbArg(usize, usize),
}

impl std::error::Error for Error {}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::FileUnreadable => write!(f, "Le fichier est illisible."),
            Error::FileUnwritable => write!(f, "Le fichier est inaccessible."),
            Error::CorruptedPng => write!(f, "Le fichier est corrompu."),
            Error::IncorrectKey(key) => write!(f, "La clé {key} est incorrecte."),
            Error::UnknownCommand(command) => write!(f, "La commande {command} est inconnue."),
            Error::NoMessageFound => write!(f, "Aucun message trouvé."),
            Error::DeletionFailure => write!(f, "Échec de la suppression du message."),
            Error::NoCommand => write!(f, "Aucune commande fournie."),
            Error::IncorrectNbArg(nb_input, nb_correct) => write!(
                f,
                "Seul {nb_input} arguments ont été donné alors que {nb_correct} sont attendus."
            ),
        }
    }
}
