// The engine's door (crates/retold-ffi). Keep in step with crates/retold-ffi/src/lib.rs.
#ifndef RETOLD_H
#define RETOLD_H

#include <stddef.h>
#include <stdint.h>

typedef struct RetoldSession RetoldSession;

// Game numbers: 1 Red, 2 Blue, 3 Yellow, 4 Gold, 5 Silver, 6 Crystal, 7 FireRed, 8 LeafGreen,
// 9 Ruby, 10 Sapphire, 11 Emerald. 0 means "not a supported cartridge".
int32_t retold_find_cartridge(const uint8_t *data, size_t len, uint8_t **out, size_t *out_len);
void retold_free_buffer(uint8_t *ptr, size_t len);
int32_t retold_game_playable(int32_t game);

RetoldSession *retold_open(const uint8_t *data, size_t len);
void retold_close(RetoldSession *s);
void retold_set_view(RetoldSession *s, uint32_t tiles_w, uint32_t tiles_h);
void retold_set_option(RetoldSession *s, uint32_t key, int32_t value);
// Buttons: 1 A, 2 B, 4 Select, 8 Start, 16 Right, 32 Left, 64 Up, 128 Down.
int32_t retold_step(RetoldSession *s, uint32_t buttons);
const uint32_t *retold_pixels(RetoldSession *s, uint32_t *width, uint32_t *height);
size_t retold_save(RetoldSession *s, uint8_t *out, size_t cap);
int32_t retold_load(RetoldSession *s, const uint8_t *data, size_t len);

// Option keys for retold_set_option (crates/retold-core/src/options.rs).
#define RETOLD_OPT_TEXT_SPEED 1
#define RETOLD_OPT_HOLD_TO_SPEED_TEXT 2
#define RETOLD_OPT_WALK_SPEED 3
#define RETOLD_OPT_RUN_SPEED 4
#define RETOLD_OPT_RUN_WITH_B 5
#define RETOLD_OPT_COLORS 6
#define RETOLD_OPT_FADES 7

#endif
