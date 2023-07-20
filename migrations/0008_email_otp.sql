-- Table: public.email_otp

-- DROP TABLE IF EXISTS public.email_otp;

CREATE SEQUENCE email_otp_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;

CREATE TABLE IF NOT EXISTS public.email_otp
(
    id integer NOT NULL DEFAULT nextval('email_otp_id_seq'::regclass),
    user_id integer NOT NULL,
    otp character varying COLLATE pg_catalog."default" NOT NULL,
    verify_url character varying COLLATE pg_catalog."default" NOT NULL,
    expires_at timestamp with time zone NOT NULL,
    created_at timestamp with time zone NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT email_otp_pkey PRIMARY KEY (id),
    CONSTRAINT email_otp_verify_url_verify_url1_key UNIQUE (verify_url),
    CONSTRAINT user_link FOREIGN KEY (user_id)
        REFERENCES public.users (id) MATCH SIMPLE
        ON UPDATE CASCADE
        ON DELETE CASCADE
)

TABLESPACE pg_default;

ALTER TABLE IF EXISTS public.email_otp
    OWNER to pg_database_owner;