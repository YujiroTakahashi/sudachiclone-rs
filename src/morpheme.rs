use std::cell::RefCell;
use std::iter::FromIterator;
use std::rc::Rc;

use super::dictionary_lib::grammar::Grammar;
use super::dictionary_lib::word_info::WordInfo;
use super::lattice_node::LatticeNode;
use super::utf8_input_text::UTF8InputText;

pub struct Morpheme {
  input_text: Rc<RefCell<UTF8InputText>>,
  word_info: WordInfo,
  grammar: Rc<RefCell<Grammar>>,
  node: Rc<RefCell<LatticeNode>>,
}

impl Morpheme {
  pub fn new(
    input_text: Rc<RefCell<UTF8InputText>>,
    word_info: WordInfo,
    grammar: Rc<RefCell<Grammar>>,
    node: Rc<RefCell<LatticeNode>>,
  ) -> Morpheme {
    Morpheme {
      input_text,
      word_info,
      grammar,
      node,
    }
  }
  pub fn surface(&self) -> String {
    let input_text = self.input_text.borrow();
    let original_text = input_text.get_original_text();
    let start = input_text.get_original_index(self.node.borrow().get_start());
    let end = input_text.get_original_index(self.node.borrow().get_end());
    String::from_iter(original_text.chars().skip(start).take(end - start))
  }
  pub fn part_of_speech(&self) -> Vec<String> {
    let grammar = self.grammar.borrow();
    grammar
      .get_part_of_speech_string(self.get_word_info().pos_id as usize)
      .clone()
  }
  pub fn part_of_speech_id(&self) -> i16 {
    self.get_word_info().pos_id
  }
  pub fn dictionary_form(&self) -> &str {
    &self.get_word_info().dictionary_form
  }
  pub fn normalized_form(&self) -> &str {
    &self.get_word_info().normalized_form
  }
  pub fn reading_form(&self) -> &str {
    &self.get_word_info().reading_form
  }
  pub fn is_oov(&self) -> bool {
    self.node.borrow().is_oov()
  }
  pub fn get_word_info(&self) -> &WordInfo {
    &self.word_info
  }
  pub fn get_word_id(&self) -> usize {
    self.node.borrow().get_word_id()
  }
  pub fn dictionary_id(&self) -> Option<usize> {
    self.node.borrow().get_dictionary_id()
  }
}