NAME = debug_build
CXX = g++
CXXFLAG = -I/usr/include/SDL2 -D_REENTRANT
DBFLAG = -O0 -ggdb3
LDFLAG = -lSDL2
files = \
		main
#files	
SRC = $(addsuffix .cpp,$(files))

.PHONY: all re fclean

.SILENT:

all: $(NAME)

$(NAME): $(SRC)
	$(CXX) $(CXXFLAG) $(DBFLAG) $^ $(LDFLAG) -o $@

clean:
	*.o

fclean: clean
	$(RM) $(NAME)

re: fclean all
