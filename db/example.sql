CREATE TABLE public.posts (
	id int4 NOT NULL GENERATED ALWAYS AS IDENTITY,
	title varchar NULL,
	"text" text NULL,
	CONSTRAINT "PK_posts" PRIMARY KEY (id)
);