-- Create quotes table
CREATE TABLE quotes (
    id TEXT PRIMARY KEY NOT NULL,
    text TEXT NOT NULL,
    author TEXT NOT NULL,
    source TEXT NOT NULL
);

-- Create tags table for quote categorization  
CREATE TABLE tags (
    quote_id TEXT NOT NULL,
    tag TEXT NOT NULL,
    PRIMARY KEY (quote_id, tag),
    FOREIGN KEY (quote_id) REFERENCES quotes(id)
);
