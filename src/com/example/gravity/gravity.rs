#pragma version(1)
#pragma rs java_package_name(com.example.gravity)
#pragma stateFragment(parent)

#include "rs_graphics.rsh"

static int newPart = 0;
static int initialized = 0;

float gTouchX1 = -1.f;
float gTouchY1 = -1.f;

float gTouchX2 = -1.f;
float gTouchY2 = -1.f;

bool wrap = false;
bool multiple = false;
float redRS = 0.5f;
float greenRS = 0.9f;
float blueRS = 0.9f;
bool blackRS = true;

float delta = 0.96f;

float acceleration = 100.f;

typedef struct __attribute__((packed, aligned(4))) Point {
	float2 delta;
	float2 position;
	uchar4 color;
} Point_t;

Point_t *point;

rs_mesh partMesh;

/**
 * Initialize the particles
 */

void initParticles() {
	int size = rsAllocationGetDimX(rsGetAllocation(point));
	float width = rsgGetWidth();
	float height = rsgGetHeight();
	uchar4 c = rsPackColorTo8888(redRS, greenRS, blueRS);
	Point_t *p = point;
	for (int i = 0; i < size; i++) {
		p->position.x = rsRand(width);
		p->position.y = rsRand(height);
		p->delta.x = 0;
		p->delta.y = 0;
		p->color = c;
		p++;
	}
	initialized = 1;
}

/**
 * root() is called every time a frame refresh occurs
 */
int root() {

	float width = rsgGetWidth();
	float height = rsgGetHeight();
//Background Switch 
	if (blackRS) {
		rsgClearColor(0.0f, 0.0f, 0.0f, 1.f);
	} else {
		rsgClearColor(1.0f, 1.0f, 1.0f, 1.f);
	}
	int size = rsAllocationGetDimX(rsGetAllocation(point));
	Point_t *p = point;

if (multiple) {
// If wrap true
		if (wrap) {
			for (int i = 0; i < size; i++) {
				if (gTouchX1 != -1) {
					float diff_x = gTouchX1 - p->position.x;
					float diff_y = gTouchY1 - p->position.y;
					float acc = acceleration
							/ (diff_x * diff_x + diff_y * diff_y);
					float acc_x = acc * diff_x;
					float acc_y = acc * diff_y;

					p->delta.x += acc_x;
					p->delta.y += acc_y;

					// This is friction
					p->delta.x *= delta;
					p->delta.y *= delta;
				}

				p->position.x += p->delta.x;
				p->position.y += p->delta.y;
				
				if (gTouchX2 != -1) {
					float diff_x = gTouchX2 - p->position.x;
					float diff_y = gTouchY2 - p->position.y;
					float acc = acceleration
							/ (diff_x * diff_x + diff_y * diff_y);
					float acc_x = acc * diff_x;
					float acc_y = acc * diff_y;

					p->delta.x += acc_x;
					p->delta.y += acc_y;

					// This is friction
					p->delta.x *= delta;
					p->delta.y *= delta;
				}

				p->position.x += p->delta.x;
				p->position.y += p->delta.y;

				uchar4 c = rsPackColorTo8888(redRS, greenRS, blueRS);
				p->color = c;

				if (p->position.x > width)
					p->position.x = 0;
				else if (p->position.x < 0)
					p->position.x = width;

				if (p->position.y > height)
					p->position.y = 0;
				else if (p->position.y < 0)
					p->position.y = height;
				p++;
			}
		}
		// If no wrap
		else {
			for (int i = 0; i < size; i++) {
				if (gTouchX1 != -1) {
					float diff_x = gTouchX1 - p->position.x;
					float diff_y = gTouchY1 - p->position.y;
					float acc = acceleration
							/ (diff_x * diff_x + diff_y * diff_y);
					float acc_x = acc * diff_x;
					float acc_y = acc * diff_y;

					p->delta.x += acc_x;
					p->delta.y += acc_y;

					// This is friction
					p->delta.x *= delta;
					p->delta.y *= delta;
				}

				p->position.x += p->delta.x;
				p->position.y += p->delta.y;

				if (gTouchX2 != -1) {
					float diff_x = gTouchX2 - p->position.x;
					float diff_y = gTouchY2 - p->position.y;
					float acc = acceleration
							/ (diff_x * diff_x + diff_y * diff_y);
					float acc_x = acc * diff_x;
					float acc_y = acc * diff_y;

					p->delta.x += acc_x;
					p->delta.y += acc_y;

					// This is friction
					p->delta.x *= delta;
					p->delta.y *= delta;
				}

				uchar4 c = rsPackColorTo8888(redRS, greenRS, blueRS);
				p->color = c;

//		Bounce
				if (p->position.x > width) {
					p->delta.x = 0 - p->delta.x;
				} else if (p->position.x < 0) {
					p->delta.x = 0 - p->delta.x;
				}

				if (p->position.y > height) {
					p->delta.y = 0 - p->delta.y;
				} else if (p->position.y < 0) {
					p->delta.y = 0 - p->delta.y;
				}
				p++;

			}
		}
	}
else {
// If wrap true
		if (wrap) {
			for (int i = 0; i < size; i++) {
				if (gTouchX1 != -1) {
					float diff_x = gTouchX1 - p->position.x;
					float diff_y = gTouchY1 - p->position.y;
					float acc = acceleration
							/ (diff_x * diff_x + diff_y * diff_y);
					float acc_x = acc * diff_x;
					float acc_y = acc * diff_y;

					p->delta.x += acc_x;
					p->delta.y += acc_y;

					// This is friction
					p->delta.x *= delta;
					p->delta.y *= delta;
				}

				p->position.x += p->delta.x;
				p->position.y += p->delta.y;

				uchar4 c = rsPackColorTo8888(redRS, greenRS, blueRS);
				p->color = c;

				if (p->position.x > width)
					p->position.x = 0;
				else if (p->position.x < 0)
					p->position.x = width;

				if (p->position.y > height)
					p->position.y = 0;
				else if (p->position.y < 0)
					p->position.y = height;
				p++;
			}
		}
		// If no wrap
		else {
			for (int i = 0; i < size; i++) {
				if (gTouchX1 != -1) {
					float diff_x = gTouchX1 - p->position.x;
					float diff_y = gTouchY1 - p->position.y;
					float acc = acceleration
							/ (diff_x * diff_x + diff_y * diff_y);
					float acc_x = acc * diff_x;
					float acc_y = acc * diff_y;

					p->delta.x += acc_x;
					p->delta.y += acc_y;

					// This is friction
					p->delta.x *= delta;
					p->delta.y *= delta;
				}

				p->position.x += p->delta.x;
				p->position.y += p->delta.y;

				uchar4 c = rsPackColorTo8888(redRS, greenRS, blueRS);
				p->color = c;

//		Bounce
				if (p->position.x > width) {
					p->delta.x = 0 - p->delta.x;
				} else if (p->position.x < 0) {
					p->delta.x = 0 - p->delta.x;
				}

				if (p->position.y > height) {
					p->delta.y = 0 - p->delta.y;
				} else if (p->position.y < 0) {
					p->delta.y = 0 - p->delta.y;
				}
				p++;

			}
		}
	}
	rsgDrawMesh(partMesh);

	return 30;
}
