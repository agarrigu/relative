NAME = debug_build
CXX = g++
CXXFLAG	= -O0 -ggdb3
files = \
		main
#files	
SRC = $(addsuffix .cpp,$(files))

.PHONY: all re fclean

.SILENT:

all: $(NAME)

$(NAME): $(SRC)
	$(CXX) $(CXXFLAG) $^ -o $@ -lSDL2

clean:
	*.o

fclean: clean
	$(RM) $(NAME)

re: fclean all
