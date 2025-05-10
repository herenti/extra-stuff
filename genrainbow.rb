#Made by chatgpt. This is for personal use only. All rights belong to the owner.

class Rainbow

    def hsl_to_rgb(h, s, l)
        c = (1 - (2 * l - 1).abs) * s
        x = c * (1 - ((h / 60.0) % 2 - 1).abs)
        m = l - c / 2.0

        r, g, b = case h
        when 0...60
            [c, x, 0]
        when 60...120
            [x, c, 0]
        when 120...180
            [0, c, x]
        when 180...240
            [0, x, c]
        when 240...300
            [x, 0, c]
        when 300...360
            [c, 0, x]
        end

        [(r + m) * 255, (g + m) * 255, (b + m) * 255].map { |val| val.round }
    end

    def rgb_to_hex(r, g, b)
        "#%02x%02x%02x" % [r, g, b]
    end

    def rainbow_text(input)
        length = input.length
        result = ""

        input.chars.each_with_index do |char, i|
            hue = (360.0 * i / length) % 360
            r, g, b = hsl_to_rgb(hue, 1.0, 0.5)
            color = rgb_to_hex(r, g, b)
            result += "<font color=\"#{color}\">#{char}</font>"
        end

        return result
    end
end


