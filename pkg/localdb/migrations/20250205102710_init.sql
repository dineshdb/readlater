CREATE TABLE [items] (
   [id] INTEGER PRIMARY KEY AUTOINCREMENT,
   [pocket_id] INTEGER UNIQUE DEFAULT NULL,
   [title] TEXT NOT NULL DEFAULT '',
   [url] TEXT NOT NULL UNIQUE,
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
	[item_id] INTEGER NOT NULL REFERENCES items(id),
	[tag_id] INTEGER NOT NULL REFERENCES tags(id),
   PRIMARY KEY (tag_id, item_id)
);

CREATE TABLE [authors] (
   [id] INTEGER PRIMARY KEY AUTOINCREMENT,
   [name] TEXT,
   [url] TEXT UNIQUE
);

CREATE TABLE [items_authors] (
   [author_id] INTEGER REFERENCES authors(id),
   [item_id] INTEGER REFERENCES items(id),
   PRIMARY KEY (author_id, item_id)
);

CREATE TABLE [kv] (
   [key] TEXT PRIMARY KEY,
   [value] TEXT,
   [updated_at] INTEGER DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE [images] (
   [id] INTEGER PRIMARY KEY AUTOINCREMENT,
   [src] TEXT NOT NULL UNIQUE,
   [width] INTEGER NOT NULL DEFAULT 0,
   [height] INTEGER NOT NULL DEFAULT 0,
   [caption] TEXT,
   [credit] TEXT
);

CREATE TABLE [items_images] (
   [image_id] INTEGER REFERENCES images(id),
   [item_id] INTEGER REFERENCES items(id),
   PRIMARY KEY (image_id, item_id)
);

CREATE TABLE [videos] (
   [id] INTEGER PRIMARY KEY AUTOINCREMENT,
   [pocket_id] TEXT NOT NULL UNIQUE,
   [src] TEXT NOT NULL UNIQUE,
   [width] INTEGER NOT NULL DEFAULT 0,
   [height] INTEGER NOT NULL DEFAULT 0,
   [kind] TEXT,
   [vid] TEXT
);

CREATE TABLE [items_videos] (
   [video_id] INTEGER REFERENCES videos(id),
   [item_id] INTEGER REFERENCES items(id),
   PRIMARY KEY (video_id, item_id)
);
