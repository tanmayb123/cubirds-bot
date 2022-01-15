use strum_macros::EnumIter;

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone, EnumIter)]
pub enum Bird {
    PARROT,
    WARBLER,
    TOUCAN,
    FLAMINGO,
    ROBIN,
    MAGPIE,
    OWL,
    DUCK
}

#[derive(Debug)]
pub struct BirdInfo {
    pub cards: i32,
    pub small: i32,
    pub large: i32,
}

impl Bird {
    pub fn from_char(character: char) -> Option<Bird> {
        match character {
            'P' => Some(Bird::PARROT),
            'W' => Some(Bird::WARBLER),
            'T' => Some(Bird::TOUCAN),
            'F' => Some(Bird::FLAMINGO),
            'R' => Some(Bird::ROBIN),
            'M' => Some(Bird::MAGPIE),
            'O' => Some(Bird::OWL),
            'D' => Some(Bird::DUCK),
            _ => None
        }
    }

    pub fn to_char(&self) -> char {
        match self {
            Bird::PARROT => 'P',
            Bird::WARBLER => 'W',
            Bird::TOUCAN => 'T',
            Bird::FLAMINGO => 'F',
            Bird::ROBIN => 'R',
            Bird::MAGPIE => 'M',
            Bird::OWL => 'O',
            Bird::DUCK => 'D',
        }
    }

    pub fn from_string(string: &str) -> Option<Vec<Bird>> {
        let mut birds = Vec::new();
        for i in string.chars() {
            if let Some(x) = Bird::from_char(i) {
                birds.push(x);
            } else {
                return None;
            }
        }
        return Some(birds);
    }

    pub fn information(self) -> BirdInfo {
        match self {
            Bird::PARROT => BirdInfo{
                cards: 13,
                small: 4,
                large: 6,
            },
            Bird::WARBLER => BirdInfo{
                cards: 20,
                small: 6,
                large: 9,
            },
            Bird::TOUCAN => BirdInfo{
                cards: 10,
                small: 3,
                large: 4,
            },
            Bird::FLAMINGO => BirdInfo{
                cards: 7,
                small: 2,
                large: 3,
            },
            Bird::ROBIN => BirdInfo{
                cards: 20,
                small: 6,
                large: 9,
            },
            Bird::MAGPIE => BirdInfo{
                cards: 17,
                small: 5,
                large: 7,
            },
            Bird::OWL => BirdInfo{
                cards: 10,
                small: 3,
                large: 4,
            },
            Bird::DUCK => BirdInfo{
                cards: 13,
                small: 4,
                large: 6,
            },
        }
    }
}
