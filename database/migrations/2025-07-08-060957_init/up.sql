CREATE TABLE IF NOT EXISTS vertretungen (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    klasse VARCHAR(5) NOT NULL,
    stufe SMALLINT CHECK (stufe >= 5 AND stufe <= 13) NOT NULL,
    stunde SMALLINT CHECK (stunde > 0 AND stunde < 10) NOT NULL,
    fach VARCHAR(10) NOT NULL,
    fach_neu VARCHAR(10),
    raum VARCHAR(10),
    raum_neu VARCHAR(10),
    lehrer VARCHAR(10),
    lehrer_neu VARCHAR(10),
    text VARCHAR(100),
    datum TIMESTAMP WITH TIME ZONE NOT NULL,
    erstelldatum TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL
);

CREATE TABLE IF NOT EXISTS infos (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    text TEXT NOT NULL,
    datum TIMESTAMP WITH TIME ZONE NOT NULL,
    erstelldatum TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL
);
