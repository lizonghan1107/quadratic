const path = require('path');
const generateBMFont = require('msdf-bmfont-xml');
const { exit } = require('process');
const fse = require('fs-extra');

const fontDirectory = path.join('public', 'fonts');
const fontFamilies = ['opensans'];
const fontFiles = ['OpenSans', 'OpenSans-Bold', 'OpenSans-Italic', 'OpenSans-BoldItalic'];

async function cleanOldFiles() {
  console.log('Removing old files...');
  for (const family of fontFamilies) {
    const files = await fse.readdir(path.join(fontDirectory, family));
    for (const file of files) {
      if (file.includes('.png') || (file.includes('.fnt'))) {
        await fse.unlink(path.join(fontDirectory, family, file));
      }
    }
  }
}

async function writeFontFiles(font, family, textures, fontData) {
  for (const texture of textures) {
    try {
      console.log(`Writing ${texture.filename}.png...`)
      await fse.writeFile(`${texture.filename}.png`, texture.texture);
    } catch (e) {
      console.error(e);
      exit(1);
    }
  }
  try {
    console.log(`Writing ${fontData.filename}...`);
    await fse.writeFile(fontData.filename, fontData.data);
  } catch (e) {
    console.error(e);
    exit(1);
  }
}

function convertFont(family, font) {
  return new Promise(async (resolve) => {
    console.log(`  ${font}...`);
    const fontFile = path.join(fontDirectory, family, `${font}.ttf`);
    generateBMFont(fontFile, { 'smart-size': true }, async (error, textures, fontData) => {
      if (error) {
        console.error(error);
        exit(1);
      }
      await writeFontFiles(font, family, textures, fontData);
      resolve();
    });
  });
}

async function convertFonts() {
  console.log('\nConverting fonts from ttf to msdf format...')

  await cleanOldFiles();

  for (const family of fontFamilies) {
    console.log(` ${family}/`);
    for (const font of fontFiles) {
      await convertFont(family, font);
    }
  }

  console.log('Conversion complete.\n');
  exit(0);
}

convertFonts();