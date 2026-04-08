// Experimental:
// A port of the proposed specification from https://wiki.epfl.ch/bigdata2015-crosswords/format

struct Crossword {
    words: Vec<WordClue>,
    title: Option<String>,
    url: Option<String>,
    source: Option<String>,
    categories: Option<Vec<String>>,
    author: Option<String>,
    difficulty: Option<i8>,   // 0 - 10 (easy - hard)
    date: Option<String>,     // date string
    language: Option<String>, // one of select codes
}

enum WordDirection {
    NORTH,
    SOUTH,
    EAST,
    WEST,
}

struct WordClue {
    word: String,
    clue: String,
    x: i8,
    y: i8,
    directorion: WordDirection,
}

struct Definition {
    word: String,
    equivalents: Option<Vec<String>>,
    associtaed: Option<Vec<String>>,
    definitions: Option<Vec<String>>,
    examples: Option<Vec<String>>,
}
