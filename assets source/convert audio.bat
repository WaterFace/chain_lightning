@echo off
ffmpeg -i originals/reload-123781.mp3 -c:a libvorbis -q:a 4 ../assets/sounds/reload.ogg
ffmpeg -i originals/single-gunshot-53-101733.mp3 -c:a libvorbis -q:a 4 ../assets/sounds/gunshot.ogg
ffmpeg -i originals/teeth-snapping-64629.mp3 -c:a libvorbis -q:a 4 ../assets/sounds/teeth.ogg
ffmpeg -i originals/explosion-312361.mp3 -c:a libvorbis -q:a 4 ../assets/sounds/explosion.ogg
ffmpeg -i originals/scifi-anime-whoosh-39-205026.mp3 -c:a libvorbis -q:a 4 ../assets/sounds/portal.ogg
ffmpeg -i originals/oof-97698.mp3 -c:a libvorbis -q:a 4 ../assets/sounds/oof.ogg
ffmpeg -i "originals/Soundtrack 4 Life - BRUTAL TIME.mp3" -c:a libvorbis -q:a 4 -b:a 64k "../assets/music/BRUTAL TIME.ogg"
