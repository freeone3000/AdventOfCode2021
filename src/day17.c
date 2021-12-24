//
// Co-written with @jodiethemathgenius.
//

#include <stdio.h>

#define PUZZLE_INPUT 1

#ifdef PUZZLE_INPUT
const int maxx=125;
const int minx=88;
const int maxy=-103;
const int miny=-157;
const int MAX = 10000;
#else
const int maxx = 30;
const int minx = 20;
const int maxy = -5;
const int miny = -10;
const int MAX = 10;
#endif

typedef int bool;
int abs(int x) { return x < 0 ? -x : x; }
int sign(int x) { return x < 0 ? -1 : x > 0 ? 1 : 0; }

bool solved(int x, int y) {
    return abs(x) >= abs(minx) && abs(x) <= abs(maxx) && y >= miny && y <= maxy;
}

bool try_entry(int dx, int dy) {
    int x = 0;
    int y = 0;
    while(y>=miny && abs(x)<=abs(maxx)){
        // inlined from step()
        x += dx;
        y += dy;
        if(dx > 0) { dx--; } else if(dx < 0) { dx++; }
        --dy;
        // end step()

        //printf("%d(%d), %d(%d)\n", x, dx, y, dy);
        if(solved(x, y)) {
            return 1;
        }
    }
    return 0;
}

int main() {
    int solutions = 0;
    for(int deltay = MAX; deltay>=miny; --deltay) {
        for(int deltax = maxx; deltax; --deltax) {
#ifndef PUZZLE_INPUT
            printf("Trying %d %d\n", deltax, deltay);
#endif // PUZZLE_INPUT
            int maxY = try_entry(deltax, deltay);
            if(maxY) {
                ++solutions;
                // print in a nice table
#ifndef PUZZLE_INPUT
                printf("%d,%d\t", deltax, deltay);
                                if(solutions % 10 == 0) { printf("%s", "\n"); }
#endif //PUZZLE_INPUT
            }
        }
    }
    printf("\n\n%d\n", solutions);
}