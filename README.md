# THW Stempelkarten

**Generieren von THW Stempelkarten aus TOML Dateien.**

Das Programm erlaubt es aus strukturierten Helferdaten Stempelkarten für die THW Stempeluhren zu generieren. Die Helferdaten werden dazu durch eine anpassbare Vorlage in ein einzelnes PDF umgewandelt. Das genaue Aussehen des PDFs kann durch Anpassen der Vorlage verändert werden. Die Vorlage in `template` erzeugt DIN-A4 Seiten mit bis zu 10 Stempelkarten pro Seite.

![Beispiel einer Stempelkarte](examples/example.png)

Unter [examples/example.pdf](examples/example.pdf) ist ein beispielhaftes PDF hinterlegt.

## Verwendung
Das Programm verfügt über keine grafische Oberfläche und muss über die Kommandozeile bedient werden. Alternativ kann auch ein Skript erstellt werden, welche die notwendigen Parameter definiert.

```
$ thw-stempelkarten --help
Usage: thw-stempelkarten [OPTIONS] --volunteer-dir <VOLUNTEER_DIR> --picture-dir <PICTURE_DIR> --template-dir <TEMPLATE_DIR>

Options:
  -v, --volunteer-dir <VOLUNTEER_DIR>  The directory containing all of the volunteer description toml files
  -p, --picture-dir <PICTURE_DIR>      The directory in which the pictures of the volunteers are placed
  -t, --template-dir <TEMPLATE_DIR>    The directory containt the template including all assets like fonts and packages
  -o, --output <OUTPUT>                The output file name under which the generated PDF is save to
  -h, --help                           Print help
```

Die Helferdaten müssen in einzelnen `.toml`-Dateien in dem mit dem Parameter `-v` angegebenen Ordner abgelegt werden. Die referenzierten Bilder in den Helferdaten müssen in dem mit `-p` angegebenen Ordner abgelegt werden. Die Vorlage für die Stempelkarten muss in dem mit `-t` angegebenem Ordner abgelegt werden. Die generierten Stempelkarten werden an dem mit `-o` angegebenem Pfad und Dateinamen gespeichert.

Die vorhandenen Beispieldaten können mit folgendem Befehl in Stempelkarten umgewandelt werden:
```bash
thw-stempelkarten -v examples/volunteers -p examples/pictures -t template -o example.pdf
```

## Helferdaten
Die Helferdaten werden für jeden Helfer in einere eigenen TOML Datei abgespeichert. Das erlaubt es durch verschieben der Dateien aus einem Archiv-Ordner in einen Druck Ordner zu definieren, für welche Helfer eine neue Stempelkarte ausgestellt werden soll. In dem folgenden Beispiel einer Helferdatei sind alle verfügbaren Felder und mit Kommentaren (Zeilen, welche mit `#` anfangen) versehen. Diese können weggelassen werden. Weitere Informationen zu dem TOML-Format sind über im Internet verfügbar.

```toml
# Vorname und Nachname des Helfers
first_name = "Max"
last_name = "Mustermann"

# Helferausweis Nummer
barcode = "12345678-90"

# Einsatzbefähigung und ob diese angezeigt werden soll
qualified = true
hide_qualified = false

# Pfad an dem das Helferbild abelegt ist innerhalb des Bilder-Ordners
picture = "helfer_bild.png"

# Dislozierung von oberster OE hin zu StAN-Position
deployment = ["TZ", "FGr. N", "Fachhelfer"]

# Unsortierte Liste an Zusatzqualifikationen
qualifications = [
    "AGT",
    "Motorsäge A/B",
    "Thermisches Trennen",
    "Schweißen",
]

# Unsortierte Liste an Fahrerlaubnissen
licenses = ["KF CE", "Staplerschein"]
```

## Stempelkarten Vorlage
Die Stempelkarten Vorlage ist in Typst geschrieben und erstellt aus den eingelesenen Helferdaten die eigentlichen Stempelkarten und ordnet diese auf Seiten an. Der Vorlagenordner muss folgende Verzeichnisse und Dateien aufweisen:
 - `template.typ` ist der Einstiegspunkt der Typst Vorlage.
 - `packages` ist der Ordner in dem verwendete Typst Pakete abgelegt werden können.
 - `fonts` ist der Ordner, in dem alle verwendeten Schriftarten abgelegt werden müssen.

`template.typ` wird vom Typst Compiler mit allen eingelesenen Helferdaten aufgerufen. Auf diese Helferdaten kann über `#sys.inputs.volunteers` zugegriffen werden. Dabei ist `volunteers` ist ein Array an Helferdaten welche wiederum folgende Felder enthalten:
 - `last_name` ist ein String mit dem Nachnamen des Helfers
 - `first_name` ist ein String mit dem Vornamen des Helfers
 - `qualified` ist ein Boolean, der angibt ob ein Helfer aktuell Einsatzbefähigt ist (`true`) oder nicht (`false`)
 - `hide_qualified` ist ein Boolean der angibt ob die Einsatzbefähigung dargestellt werden soll (`true`) oder nicht (`false`)
 - `deployment` ist ein Array an Strings mit der Dislozierung des Helfers von höchster Organisationeinheit (OE) über niedrigere OEs bis zu der StAN-Position (z.B. Fachhelfer)
 - `licenses` ist ein Array an Strings mit den erworbenen Fahrerlaubnissen
 - `qualifications` ist ein Array an Strings mit den erworbenen Zusatzqualifikationen (z.B. AGT, Motorsäge A/B)
 - `barcode` ist ein String mit dem Inhalt des Barcodes wie er auch auf dem Helferausweis angegeben ist (z.B. `12345678-90`).
 - `picture` ist ein String mit dem Pfad an dem das Bild des Helfers abgelegt ist.

