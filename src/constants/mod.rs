pub const META_EOF: u16 = 0x0000;
pub const META_REALIZEPALETTE: u16 = 0x0035;
pub const META_SETPALENTRIES: u16 = 0x0037;
pub const META_SETBKMODE: u16 = 0x0102;
pub const META_SETMAPMODE: u16 = 0x0103;
pub const META_SETROP2: u16 = 0x0104;
pub const META_SETRELABS: u16 = 0x0105;
pub const META_SETPOLYFILLMODE: u16 = 0x0106;
pub const META_SETSTRETCHBLTMODE: u16 = 0x0107;
pub const META_SETTEXTCHAREXTRA: u16 = 0x0108;
pub const META_RESTOREDC: u16 = 0x0127;
pub const META_RESIZEPALETTE: u16 = 0x0139;
pub const META_DIBCREATEPATTERNBRUSH: u16 = 0x0142;
pub const META_SETLAYOUT: u16 = 0x0149;
pub const META_SETBKCOLOR: u16 = 0x0201;
pub const META_SETTEXTCOLOR: u16 = 0x0209;
pub const META_OFFSETVIEWPORTORG: u16 = 0x0211;
pub const META_LINETO: u16 = 0x0213;
pub const META_MOVETO: u16 = 0x0214;
pub const META_OFFSETCLIPRGN: u16 = 0x0220;
pub const META_FILLREGION: u16 = 0x0228;
pub const META_SETMAPPERFLAGS: u16 = 0x0231;
pub const META_SELECTPALETTE: u16 = 0x0234;
pub const META_POLYGON: u16 = 0x0324;
pub const META_POLYLINE: u16 = 0x0325;
pub const META_SETTEXTJUSTIFICATION: u16 = 0x020A;
pub const META_SETWINDOWORG: u16 = 0x020B;
pub const META_SETWINDOWEXT: u16 = 0x020C;
pub const META_SETVIEWPORTORG: u16 = 0x020D;
pub const META_SETVIEWPORTEXT: u16 = 0x020E;
pub const META_OFFSETWINDOWORG: u16 = 0x020F;
pub const META_SCALEWINDOWEXT: u16 = 0x0410;
pub const META_SCALEVIEWPORTEXT: u16 = 0x0412;
pub const META_EXCLUDECLIPRECT: u16 = 0x0415;
pub const META_INTERSECTCLIPRECT: u16 = 0x0416;
pub const META_ELLIPSE: u16 = 0x0418;
pub const META_FLOODFILL: u16 = 0x0419;
pub const META_FRAMEREGION: u16 = 0x0429;
pub const META_ANIMATEPALETTE: u16 = 0x0436;
pub const META_TEXTOUT: u16 = 0x0521;
pub const META_POLYPOLYGON: u16 = 0x0538;
pub const META_EXTFLOODFILL: u16 = 0x0548;
pub const META_RECTANGLE: u16 = 0x041B;
pub const META_SETPIXEL: u16 = 0x041F;
pub const META_ROUNDRECT: u16 = 0x061C;
pub const META_PATBLT: u16 = 0x061D;
pub const META_SAVEDC: u16 = 0x001E;
pub const META_PIE: u16 = 0x081A;
pub const META_STRETCHBLT: u16 = 0x0B23;
pub const META_ESCAPE: u16 = 0x0626;
pub const META_INVERTREGION: u16 = 0x012A;
pub const META_PAINTREGION: u16 = 0x012B;
pub const META_SELECTCLIPREGION: u16 = 0x012C;
pub const META_SELECTOBJECT: u16 = 0x012D;
pub const META_SETTEXTALIGN: u16 = 0x012E;
pub const META_ARC: u16 = 0x0817;
pub const META_CHORD: u16 = 0x0830;
pub const META_BITBLT: u16 = 0x0922;
pub const META_EXTTEXTOUT: u16 = 0x0a32;
pub const META_SETDIBTODEV: u16 = 0x0d33;
pub const META_DIBBITBLT: u16 = 0x0940;
pub const META_DIBSTRETCHBLT: u16 = 0x0b41;
pub const META_STRETCHDIB: u16 = 0x0f43;
pub const META_DELETEOBJECT: u16 = 0x01f0;
pub const META_CREATEPALETTE: u16 = 0x00f7;
pub const META_CREATEPATTERNBRUSH: u16 = 0x01F9;
pub const META_CREATEPENINDIRECT: u16 = 0x02FA;
pub const META_CREATEFONTINDIRECT: u16 = 0x02FB;
pub const META_CREATEBRUSHINDIRECT: u16 = 0x02FC;
pub const META_CREATEREGION: u16 = 0x06FF;

/// Paints a single, constant color, either solid or dithered
pub const BS_SOLID: u16 = 0x0000;
/// Does nothing. Using a BS_NULL brush in a graphics operation MUST have the same effect as using no brush at all
pub const BS_NULL: u16 = 0x0001;
///  Paints a predefined simple pattern, or "hatch", onto a solid background
pub const BS_HATCHED: u16 = 0x0002;
/// Paints a pattern defined by a bitmap, which can be a [`Bitmap16`](crate::objects::Bitmap16) or a [`DeviceIndependentBitmap`](crate::objects::DeviceIndependentBitmap)
pub const BS_PATTERN: u16 = 0x0003;
/// Not supported
pub const BS_INDEXED: u16 = 0x0004;
/// A pattern brush specified by a [`DeviceIndependentBitmap`](crate::objects::DeviceIndependentBitmap)
pub const BS_DIBPATTERN: u16 = 0x0005;
/// A pattern brush specified by a [`DeviceIndependentBitmap`](crate::objects::DeviceIndependentBitmap)
pub const BS_DIBPATTERNPT: u16 = 0x0006;
/// Not supported
pub const BS_PATTERN8X8: u16 = 0x0007;
/// Not supported
pub const BS_DIBPATTERN8X8: u16 = 0x0008;
/// Not supported
pub const BS_MONOPATTERN: u16 = 0x0009;
