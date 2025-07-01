class LogLineParser
  def initialize(line) = @line = line

  def message = @line.split(':').last.strip
  def log_level = @line[1..@line.index(':') - 2].downcase
  def reformat = "#{message} (#{log_level})"
end
