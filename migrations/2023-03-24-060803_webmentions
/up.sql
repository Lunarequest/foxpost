-- Your SQL goes here
CREATE TABLE pending_requests (
	id		SERIAL PRIMARY KEY,
	source	TEXT UNIQUE NOT NULL,
	target	TEXT UNIQUE NOT NULL
);

CREATE TABLE completed_requests (
	id		SERIAL PRIMARY KEY,
	source	TEXT UNIQUE NOT NULL,
	target	TEXT UNIQUE NOT NULL,
	content TEXT NOT NULL,
	author	TEXT NOT NULL,
	author_url TEXT NOT NULL,
	url		TEXT NOT NULL
);
