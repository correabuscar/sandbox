/* Copyright 2021 Aristocratos (jakob@qvantnet.com)

   Licensed under the Apache License, Version 2.0 (the "License");
   you may not use this file except in compliance with the License.
   You may obtain a copy of the License at

       http://www.apache.org/licenses/LICENSE-2.0

   Unless required by applicable law or agreed to in writing, software
   distributed under the License is distributed on an "AS IS" BASIS,
   WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
   See the License for the specific language governing permissions and
   limitations under the License.

indent = tab
tab-size = 4
*/

#include <array>
#include <algorithm>
#include <cmath>
#include <ranges>
#include <stdexcept>
#include <string>
#include <utility>

#include "btop_draw.hpp"
#include "btop_config.hpp"
#include "btop_theme.hpp"
#include "btop_shared.hpp"
#include "btop_tools.hpp"
#include "btop_input.hpp"
#include "btop_menu.hpp"


using std::array;
using std::clamp;
using std::cmp_equal;
using std::cmp_greater;
using std::cmp_less;
using std::cmp_less_equal;
using std::floor;
using std::max;
using std::min;
using std::round;
using std::to_string;
using std::views::iota;

using namespace Tools;
using namespace std::literals; // for operator""s
namespace rng = std::ranges;

namespace Symbols {
    const string meter = "■";

    const array<string, 10> superscript = { "⁰", "¹", "²", "³", "⁴", "⁵", "⁶", "⁷", "⁸", "⁹" };

    const std::unordered_map<string, vector<string>> graph_symbols = {
        { "braille_up", {
            " ", "⢀", "⢠", "⢰", "⢸",
            "⡀", "⣀", "⣠", "⣰", "⣸",
            "⡄", "⣄", "⣤", "⣴", "⣼",
            "⡆", "⣆", "⣦", "⣶", "⣾",
            "⡇", "⣇", "⣧", "⣷", "⣿"
        }},
        {"braille_down", {
            " ", "⠈", "⠘", "⠸", "⢸",
            "⠁", "⠉", "⠙", "⠹", "⢹",
            "⠃", "⠋", "⠛", "⠻", "⢻",
            "⠇", "⠏", "⠟", "⠿", "⢿",
            "⡇", "⡏", "⡟", "⡿", "⣿"
        }},
        {"block_up", {
            " ", "▗", "▗", "▐", "▐",
            "▖", "▄", "▄", "▟", "▟",
            "▖", "▄", "▄", "▟", "▟",
            "▌", "▙", "▙", "█", "█",
            "▌", "▙", "▙", "█", "█"
        }},
        {"block_down", {
            " ", "▝", "▝", "▐", "▐",
            "▘", "▀", "▀", "▜", "▜",
            "▘", "▀", "▀", "▜", "▜",
            "▌", "▛", "▛", "█", "█",
            "▌", "▛", "▛", "█", "█"
        }},
        {"tty_up", {
            " ", "░", "░", "▒", "▒",
            "░", "░", "▒", "▒", "█",
            "░", "▒", "▒", "▒", "█",
            "▒", "▒", "▒", "█", "█",
            "▒", "█", "█", "█", "█"
        }},
        {"tty_down", {
            " ", "░", "░", "▒", "▒",
            "░", "░", "▒", "▒", "█",
            "░", "▒", "▒", "▒", "█",
            "▒", "▒", "▒", "█", "█",
            "▒", "█", "█", "█", "█"
        }}
    };

}

namespace Draw {

    string banner_gen(int y, int x, bool centered, bool redraw) {
        static string banner;
        static size_t width = 2;
        if (redraw) banner.clear();
        if (banner.empty()) {
            string b_color, bg, fg, oc, letter;
            auto lowcolor = Config::getB("lowcolor");
            auto tty_mode = Config::getB("tty_mode");
            for (size_t z = 1; const auto& line : Global::Banner_src) {
