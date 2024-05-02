#[doc = "Register `MIPI_IMG_DATA_SEL` reader"]
pub type R = crate::R<MipiImgDataSelSpec>;
#[doc = "Register `MIPI_IMG_DATA_SEL` writer"]
pub type W = crate::W<MipiImgDataSelSpec>;
#[doc = "Field `DATA_TYPE_SEL` reader - data type selector for image\n\ndata output: 0x08...0x0F generic\n\nshort packets\n\n0x12 embedded\n\n8-bit data 0x18 YUV\n\n420 8-bit\n\n0x19 YUV 420 10-bit\n\n0x1A Legacy YUV 420 8-bit 0x1C YUV 420 8-bit\n\n(CSPS)\n\n0x1D YUV 420 10-bit (CSPS)\n\n0x1E YUV 422 8-bit\n\n0x1F YUV 422 10-bit\n\n0x20 RGB 444\n\n0x21 RGB 555\n\n0x22 RGB 565\n\n0x23 RGB 666\n\n0x24 RGB 888\n\n0x28 RAW 6\n\n0x29 RAW 7\n\n0x2A RAW 8\n\n0x2B RAW 10\n\n0x2C RAW 12\n\n0x30...0x37 User Defined Byte-based data"]
pub type DataTypeSelR = crate::FieldReader;
#[doc = "Field `DATA_TYPE_SEL` writer - data type selector for image\n\ndata output: 0x08...0x0F generic\n\nshort packets\n\n0x12 embedded\n\n8-bit data 0x18 YUV\n\n420 8-bit\n\n0x19 YUV 420 10-bit\n\n0x1A Legacy YUV 420 8-bit 0x1C YUV 420 8-bit\n\n(CSPS)\n\n0x1D YUV 420 10-bit (CSPS)\n\n0x1E YUV 422 8-bit\n\n0x1F YUV 422 10-bit\n\n0x20 RGB 444\n\n0x21 RGB 555\n\n0x22 RGB 565\n\n0x23 RGB 666\n\n0x24 RGB 888\n\n0x28 RAW 6\n\n0x29 RAW 7\n\n0x2A RAW 8\n\n0x2B RAW 10\n\n0x2C RAW 12\n\n0x30...0x37 User Defined Byte-based data"]
pub type DataTypeSelW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - data type selector for image\n\ndata output: 0x08...0x0F generic\n\nshort packets\n\n0x12 embedded\n\n8-bit data 0x18 YUV\n\n420 8-bit\n\n0x19 YUV 420 10-bit\n\n0x1A Legacy YUV 420 8-bit 0x1C YUV 420 8-bit\n\n(CSPS)\n\n0x1D YUV 420 10-bit (CSPS)\n\n0x1E YUV 422 8-bit\n\n0x1F YUV 422 10-bit\n\n0x20 RGB 444\n\n0x21 RGB 555\n\n0x22 RGB 565\n\n0x23 RGB 666\n\n0x24 RGB 888\n\n0x28 RAW 6\n\n0x29 RAW 7\n\n0x2A RAW 8\n\n0x2B RAW 10\n\n0x2C RAW 12\n\n0x30...0x37 User Defined Byte-based data"]
    #[inline(always)]
    pub fn data_type_sel(&self) -> DataTypeSelR {
        DataTypeSelR::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - data type selector for image\n\ndata output: 0x08...0x0F generic\n\nshort packets\n\n0x12 embedded\n\n8-bit data 0x18 YUV\n\n420 8-bit\n\n0x19 YUV 420 10-bit\n\n0x1A Legacy YUV 420 8-bit 0x1C YUV 420 8-bit\n\n(CSPS)\n\n0x1D YUV 420 10-bit (CSPS)\n\n0x1E YUV 422 8-bit\n\n0x1F YUV 422 10-bit\n\n0x20 RGB 444\n\n0x21 RGB 555\n\n0x22 RGB 565\n\n0x23 RGB 666\n\n0x24 RGB 888\n\n0x28 RAW 6\n\n0x29 RAW 7\n\n0x2A RAW 8\n\n0x2B RAW 10\n\n0x2C RAW 12\n\n0x30...0x37 User Defined Byte-based data"]
    #[inline(always)]
    #[must_use]
    pub fn data_type_sel(&mut self) -> DataTypeSelW<MipiImgDataSelSpec> {
        DataTypeSelW::new(self, 0)
    }
}
#[doc = "Image Data Selector\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mipi_img_data_sel::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mipi_img_data_sel::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MipiImgDataSelSpec;
impl crate::RegisterSpec for MipiImgDataSelSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mipi_img_data_sel::R`](R) reader structure"]
impl crate::Readable for MipiImgDataSelSpec {}
#[doc = "`write(|w| ..)` method takes [`mipi_img_data_sel::W`](W) writer structure"]
impl crate::Writable for MipiImgDataSelSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MIPI_IMG_DATA_SEL to value 0"]
impl crate::Resettable for MipiImgDataSelSpec {
    const RESET_VALUE: u32 = 0;
}
