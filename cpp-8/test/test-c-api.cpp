// Copyright (C) 2020  Matthew "strager" Glazar
// See end of file for extended copyright information.

#include <gtest/gtest.h>
#include <quick-lint-js/c-api.h>
#include <quick-lint-js/i18n/translation-table.h>
#include <quick-lint-js/port/char8.h>
#include <quick-lint-js/util/algorithm.h>

namespace quick_lint_js {
namespace {
TEST(test_c_api_web_demo, empty_document_has_no_diagnostics) {
  qljs_web_demo_document* p = qljs_web_demo_create_document();
  const qljs_web_demo_diagnostic* diagnostics = qljs_web_demo_lint(p);
  EXPECT_EQ(diagnostics[0].message, nullptr);
  qljs_web_demo_destroy_document(p);
}

TEST(test_c_api_web_demo, lint_error_after_text_insertion) {
  qljs_web_demo_document* p = qljs_web_demo_create_document();

  const char8* document_text = u8"'unfinished";
  qljs_web_demo_set_text(p, document_text, strlen(document_text));
  const qljs_web_demo_diagnostic* diagnostics = qljs_web_demo_lint(p);
  EXPECT_NE(diagnostics[0].message, nullptr);
  EXPECT_EQ(diagnostics[1].message, nullptr);
  EXPECT_STREQ(diagnostics[1].code, "");

  EXPECT_STREQ(diagnostics[0].message, "unclosed string literal");
  EXPECT_STREQ(diagnostics[0].code, "E0040");
  EXPECT_EQ(diagnostics[0].begin_offset, strlen(u8""));
  EXPECT_EQ(diagnostics[0].end_offset, strlen(u8"'unfinished"));

  qljs_web_demo_destroy_document(p);
}

TEST(test_c_api_web_demo, lint_new_error_after_second_text_insertion) {
  qljs_web_demo_document* p = qljs_web_demo_create_document();

  const char8* document_text = u8"let x";
  qljs_web_demo_set_text(p, document_text, strlen(document_text));
  const qljs_web_demo_diagnostic* diagnostics = qljs_web_demo_lint(p);
  EXPECT_EQ(diagnostics[0].message, nullptr);

  const char8* document_text_2 = u8"let x = 'unfinished";
  qljs_web_demo_set_text(p, document_text_2, strlen(document_text_2));
  diagnostics = qljs_web_demo_lint(p);
  EXPECT_NE(diagnostics[0].message, nullptr);
  EXPECT_EQ(diagnostics[1].message, nullptr);
  EXPECT_STREQ(diagnostics[1].code, "");

  EXPECT_STREQ(diagnostics[0].message, "unclosed string literal");
  EXPECT_STREQ(diagnostics[0].code, "E0040");
  EXPECT_EQ(diagnostics[0].begin_offset, strlen(u8"let x = "));
  EXPECT_EQ(diagnostics[0].end_offset, strlen(u8"let x = 'unfinished"));

  qljs_web_demo_destroy_document(p);
}

TEST(test_c_api_web_demo, setting_locale_changes_messages_forever) {
  qljs_web_demo_document* p = qljs_web_demo_create_document();

  qljs_web_demo_set_locale(p, "en_US@snarky");

  const char8* document_text_1 = u8"'unfinished";
  qljs_web_demo_set_text(p, document_text_1, strlen(document_text_1));
  const qljs_web_demo_diagnostic* diagnostics = qljs_web_demo_lint(p);
  EXPECT_STREQ(diagnostics[0].message,
               "\"unclosed string literal");

  const char8* document_text_2 = u8"`unfinished";
  qljs_web_demo_set_text(p, document_text_2, strlen(document_text_2));
  diagnostics = qljs_web_demo_lint(p);
  EXPECT_STREQ(diagnostics[0].message,
               "`unclosed template");

  qljs_web_demo_destroy_document(p);
}

TEST(test_c_api, locale_list) {
  std::vector<std::string> locale_strings;
  const char* const* locales = qljs_list_locales();
  for (const char* const* l = locales; *l; ++l) {
    locale_strings.push_back(*l);
  }
  sort(locale_strings);

  std::vector<std::string> expected_locale_strings;
  for (const char* l = translation_data.locale_table; *l != '\0';
       l += std::strlen(l) + 1) {
    expected_locale_strings.push_back(l);
  }
  expected_locale_strings.push_back("");
  sort(expected_locale_strings);

  EXPECT_EQ(locale_strings, expected_locale_strings);
}
}
}

// quick-lint-js finds bugs in JavaScript programs.
// Copyright (C) 2020  Matthew "strager" Glazar
//
// This file is part of quick-lint-js.
//
// quick-lint-js is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// quick-lint-js is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with quick-lint-js.  If not, see <https://www.gnu.org/licenses/>.
