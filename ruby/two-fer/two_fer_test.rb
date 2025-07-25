# frozen_string_literal: true

require 'minitest/autorun'
require_relative 'two_fer'

class TwoFerTest < Minitest::Test
  def test_no_name_given
    assert_equal 'One for you, one for me.', TwoFer.two_fer
  end

  def test_a_name_given
    assert_equal 'One for Alice, one for me.', TwoFer.two_fer('Alice')
  end

  def test_another_name_given
    assert_equal 'One for Bob, one for me.', TwoFer.two_fer('Bob')
  end
end
