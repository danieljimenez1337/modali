CC = gcc
CFLAGS = `pkg-config --cflags gtk4 json-glib-1.0`
LIBS = `pkg-config --libs gtk4 json-glib-1.0`

modali: main.c
	$(CC) $(CFLAGS) -o modali main.c $(LIBS)

clean:
	rm -f modali

.PHONY: clean
