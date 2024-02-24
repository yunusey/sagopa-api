DROP TABLE IF EXISTS albums;
DROP TABLE IF EXISTS songs;
DROP TABLE IF EXISTS quotes;

CREATE TABLE IF NOT EXISTS albums (
	id SERIAL PRIMARY KEY,
	name TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS songs (
	id SERIAL PRIMARY KEY,
	album_id INTEGER NOT NULL,
	name TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS quotes (
	id SERIAL PRIMARY KEY,
	song_id INTEGER NOT NULL,
	quote TEXT NOT NULL
);

INSERT INTO albums (name) VALUES
	('Kağıt Kesikleri'),
	('Tek'),
	('Yunus'),
	('Sarkastik'),
	('Kalp Hastası'),
	('Saykodelik'),
	('Kötü İnsanları Tanıma Senesi'),
	('İkimizi Anlatan Bir Şey'),
	('Romantizma'),
	('Bir Pesimistin Göz Yaşları'),
	('Sözlerim Silahım'),
	('İhtiyar Heyeti'),
	('Ahmak Islatan'),
	('Underground Years'),
	('Saydam Odalar'),
	('Şarkı Koleksiyoncusu');
