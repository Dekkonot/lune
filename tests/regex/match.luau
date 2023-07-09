local RegEx = require("@lune/regex")

do
	local text = "I categorically deny having triskaidekaphobia."
	assert(RegEx.new([[\b\w{13}\b]]):ismatch(text))
end

do
	local text = "I categorically deny having triskaidekaphobia."
	local match = RegEx.new([[\b\w{13}\b]]):find(text)

	assert(
		string.sub(text, match.start, match.finish) == "categorically",
		"start and end of match didn't line up to Lua indices"
	)

	assert(match.start == 3, `start was {match.start} when it was expected to be 3`)
	assert(match.finish == 15, `finish was {match.finish} when it was expected to be 15`)
	assert(match.len == 13, `len was {match.len} when it was expected to be 13`)
	assert(match.text == "categorically", `text was {match.text} and not categorically`)
	assert(not match:isempty(), "match was empty")
end

do
	local text = "Retroactively relinquishing remunerations is reprehensible."
	local matches = RegEx.new([[\b\w{13}\b]]):findall(text)

	assert(#matches == 4, `got {#matches} when expecting 4`)

	assert(matches:get(1).text == "Retroactively")
	assert(matches:get(2).text == "relinquishing")
	assert(matches:get(3).text == "remunerations")
	assert(matches:get(4).text == "reprehensible")
end

-- do
-- 	local re = RegEx.new([['([^']+)'\s+\((\d{4})\)]])
-- 	local text = "Not my favorite movie: 'Citizen Kane' (1941)."
-- 	local caps = re:Captures(text)

-- 	assert(caps:Get(1).Text == "'Citizen Kane' (1941)")
-- 	assert(caps:Get(2).Text == "Citizen Kane")
-- 	assert(caps:Get(3).Text == "1941")
-- end

-- do
-- 	local re = RegEx.new([['(?P<title>[^']+)'\s+\((?P<year>\d{4})\)]])
-- 	local text = "Not my favorite movie: 'Citizen Kane' (1941)."
-- 	local caps = re:Captures(text)

-- 	assert(caps:Name("title").Text == "Citizen Kane")
-- 	assert(caps:Name("year").Text == "1941")
-- 	assert(caps:Get(1).Text == "'Citizen Kane' (1941)")
-- end
