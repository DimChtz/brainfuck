function[bf] = brainwords(text)

bf = '';
lastchar = 0;

for i = 1:size(text, 2)
   
   c = text(i);
    
   if c > lastchar
       factor = uint8(floor(sqrt(c - lastchar)));
       remain = uint8(c - lastchar) - factor .* factor;
       direction = '+';
   else
       factor = uint8(floor(sqrt(lastchar - c)));
       remain = uint8(lastchar - c) - factor .* factor;
       direction = '-';
   end
   
   bf = strcat(bf, repmat('+', [1 factor]));
   bf = strcat(bf, '[>');
   bf = strcat(bf, repmat(direction, [1 factor]));
   bf = strcat(bf, '<-]>');
   bf = strcat(bf, repmat(direction, [1 remain]));
   bf = strcat(bf, '.<');
   
   lastchar = c;
  
end
