CC := g++
CFLAGS := -std=c++11 -Wall -Wextra

# Directories
SRCDIR = src
BLDDIR = build
TARGET = main

SRCS = $(shell find $(SRCDIR) -name "*.cpp")
OBJS = $(patsubst $(SRCDIR)/%.cpp,$(BLDDIR)/%.o, $(SRCS))

$(TARGET): $(OBJS)
	@$(CC) $(CFLAGS) -o $@ $^

$(BLDDIR)/%.o: $(SRCDIR)/%.cpp $(SRCDIR)
	@mkdir -p $(dir $@)
	@$(CC) $(CFLAGS) -c -o $@ $<

clean:
	rm $(TARGET)
	rm -r $(BLDDIR)

run:
	./$(TARGET)
