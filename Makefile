CXX	= g++
CXXFLAGS	= -Wall -Wextra -Werror -O0 -g3 -std=c++11
LDFLAGS	= -lX11
NAME	= debug_build
BUILDDIR	= build
SRC	= \
	linux/entry.cpp \
#src
OBJ = $(SRC:.cpp=.o)
dir_guard	= mkdir -p $(@D)

.SILENT:

.PHONY: all clean fclean re

all: $(BUILDDIR)/$(NAME)

clean:
	$(RM) $(OBJ)

fclean: clean
	$(RM) $(NAME)

re: fclean all

$(BUILDDIR)/$(NAME): $(OBJ)
	$(dir_guard)
	$(CXX) $(CXXFLAGS) $^ $(LDFLAGS) -o $@

%.o: %.cpp
	$(CXX) $(CXXFLAGS) -c $< -o $@
