CREATE TABLE [items] (
   [id] INTEGER PRIMARY KEY,
   [pocket_id] INTEGER UNIQUE DEFAULT NULL,
   [title] TEXT NOT NULL DEFAULT '',
   [url] TEXT NOT NULL,
   [excerpt] TEXT,

   [is_article] INTEGER,
   [is_index] INTEGER,
   [has_video] INTEGER,
   [has_image] INTEGER,
   [word_count] INTEGER,
   [lang] TEXT,
   [images] TEXT,
   [listen_duration_estimate] INTEGER,
   [time_to_read] INTEGER,
   [top_image_url] TEXT, 
   [videos] TEXT,

   [status] INTEGER,
   [time_added] INTEGER,
   [time_updated] INTEGER,
   [time_read] INTEGER,
   [time_favorited] INTEGER
);

CREATE TABLE [tags] (
	[id] INTEGER PRIMARY KEY AUTOINCREMENT,
	[tag] TEXT NOT NULL COLLATE NOCASE UNIQUE,
   [name] TEXT
);

CREATE TABLE [items_tags] (
	[item_id] INTEGER REFERENCES items(id),
	[tag_id] INTEGER REFERENCES tags(id)
);

CREATE TABLE [authors] (
   [id] INTEGER PRIMARY KEY,
   [name] TEXT,
   [url] TEXT
);

CREATE TABLE [items_authors] (
   [author_id] INTEGER REFERENCES authors(author_id),
   [item_id] INTEGER REFERENCES items(id),
   PRIMARY KEY (author_id, item_id)
);

CREATE TABLE [kv] (
   [key] TEXT PRIMARY KEY,
   [value] TEXT,
   [updated_at] INTEGER DEFAULT CURRENT_TIMESTAMP
);
