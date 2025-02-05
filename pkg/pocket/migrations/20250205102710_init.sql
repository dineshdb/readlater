CREATE TABLE [items] (
   [id] INTEGER PRIMARY KEY,
   [given_url] TEXT,
   [given_title] TEXT,
   [favorite] INTEGER,
   [status] INTEGER,
   [time_added] INTEGER,
   [time_updated] INTEGER,
   [time_read] INTEGER,
   [time_favorited] TEXT
);

CREATE TABLE [resolved_items] (
   [id] INTEGER PRIMARY KEY,
   [title] TEXT,
   [url] TEXT,
   [excerpt] TEXT,
   [is_article] INTEGER,
   [is_index] INTEGER,
   [has_video] INTEGER,
   [has_image] INTEGER,
   [word_count] INTEGER,
   [lang] TEXT,
   [image] TEXT,
   [images] TEXT,
   [listen_duration_estimate] INTEGER,
   [time_to_read] INTEGER,
   [top_image_url] TEXT, 
   [domain_metadata] TEXT,
   [videos] TEXT
);

CREATE TABLE [tags] (
	[id] INTEGER PRIMARY KEY AUTOINCREMENT,
	[tag] TEXT NOT NULL COLLATE NOCASE UNIQUE
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
