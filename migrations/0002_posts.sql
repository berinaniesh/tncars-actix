-- DROP TABLE IF EXISTS public.posts;

CREATE SEQUENCE posts_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;

CREATE TYPE transmission_type AS ENUM ('Manual', 'Automatic', 'NotApplicable');
CREATE TYPE fuel_type AS ENUM ('Petrol', 'Diesel', 'CNG', 'Electric', 'Other');

CREATE TABLE IF NOT EXISTS public.posts
(
    id integer NOT NULL DEFAULT nextval('posts_id_seq'::regclass),
    title character varying COLLATE pg_catalog."default" NOT NULL,
    user_id integer NOT NULL,
    brand character varying COLLATE pg_catalog."default" NOT NULL,
    post_pic character varying COLLATE pg_catalog."default",
    price integer NOT NULL,
    model_year integer NOT NULL,
    km_driven integer NOT NULL,
    transmission transmission_type NOT NULL,
    fuel fuel_type NOT NULL,
    description text COLLATE pg_catalog."default" NOT NULL,
    location character varying COLLATE pg_catalog."default" NOT NULL,
    is_sold boolean NOT NULL DEFAULT false,
    created_at timestamp with time zone NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at timestamp with time zone NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT posts_pkey PRIMARY KEY (id),
    CONSTRAINT user_id FOREIGN KEY (user_id)
        REFERENCES public.users (id) MATCH SIMPLE
        ON UPDATE CASCADE
        ON DELETE CASCADE
)

TABLESPACE pg_default;

ALTER TABLE IF EXISTS public.posts
    OWNER to pg_database_owner;

COMMENT ON CONSTRAINT user_id ON public.posts
    IS 'link posts to user_id';

CREATE TRIGGER update_customer_modtime BEFORE UPDATE ON posts FOR EACH ROW EXECUTE PROCEDURE  update_modified_column();