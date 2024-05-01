#[doc = "Register `SWREG122` reader"]
pub type R = crate::R<Swreg122Spec>;
#[doc = "Register `SWREG122` writer"]
pub type W = crate::W<Swreg122Spec>;
#[doc = "Field `MFR_REG2` reader - multi format reuse register2 except h264\n\nMPEG4:\n\n\\[31:26\\]
: Exact bit of stream start word\n\n\\[25\\]
: enable for sync markers\n\n\\[24\\]
: enable for Type 1 quantization\n\n\\[23:19\\]
: the offset of Qp filter\n\n\\[18:14\\]
: the offset of Qp filter for cr\n\n\\[0\\]
: filed_pic_flag exists in stream\n\nJPEG :\n\n\\[31:26\\]
: Exact bit of stream start word\n\n\\[25\\]
: enable for sync markers\n\n\\[12:11\\]
: total of Quantization tables\n\n\\[10:8\\]
: the sampling format for input pic\n\n\\[7\\]
: JPEG width\n\n\\[6\\]
: weather current strem buffer contain the end of a\n\nJPEG image\n\n\\[5:0\\]
: vlc table\n\nvp6:\n\n\\[23:18\\]
: start bit for ctrl stream (vp7)\n\n\\[17\\]
: enable for huffman decoding\n\n\\[16\\]
: enable for muti stream (vp7)\n\n\\[15:8\\]
: boolean dec init value(vp7)\n\n\\[7:0\\]
: boolean dec init range"]
pub type MfrReg2R = crate::FieldReader<u32>;
#[doc = "Field `MFR_REG2` writer - multi format reuse register2 except h264\n\nMPEG4:\n\n\\[31:26\\]
: Exact bit of stream start word\n\n\\[25\\]
: enable for sync markers\n\n\\[24\\]
: enable for Type 1 quantization\n\n\\[23:19\\]
: the offset of Qp filter\n\n\\[18:14\\]
: the offset of Qp filter for cr\n\n\\[0\\]
: filed_pic_flag exists in stream\n\nJPEG :\n\n\\[31:26\\]
: Exact bit of stream start word\n\n\\[25\\]
: enable for sync markers\n\n\\[12:11\\]
: total of Quantization tables\n\n\\[10:8\\]
: the sampling format for input pic\n\n\\[7\\]
: JPEG width\n\n\\[6\\]
: weather current strem buffer contain the end of a\n\nJPEG image\n\n\\[5:0\\]
: vlc table\n\nvp6:\n\n\\[23:18\\]
: start bit for ctrl stream (vp7)\n\n\\[17\\]
: enable for huffman decoding\n\n\\[16\\]
: enable for muti stream (vp7)\n\n\\[15:8\\]
: boolean dec init value(vp7)\n\n\\[7:0\\]
: boolean dec init range"]
pub type MfrReg2W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - multi format reuse register2 except h264\n\nMPEG4:\n\n\\[31:26\\]
: Exact bit of stream start word\n\n\\[25\\]
: enable for sync markers\n\n\\[24\\]
: enable for Type 1 quantization\n\n\\[23:19\\]
: the offset of Qp filter\n\n\\[18:14\\]
: the offset of Qp filter for cr\n\n\\[0\\]
: filed_pic_flag exists in stream\n\nJPEG :\n\n\\[31:26\\]
: Exact bit of stream start word\n\n\\[25\\]
: enable for sync markers\n\n\\[12:11\\]
: total of Quantization tables\n\n\\[10:8\\]
: the sampling format for input pic\n\n\\[7\\]
: JPEG width\n\n\\[6\\]
: weather current strem buffer contain the end of a\n\nJPEG image\n\n\\[5:0\\]
: vlc table\n\nvp6:\n\n\\[23:18\\]
: start bit for ctrl stream (vp7)\n\n\\[17\\]
: enable for huffman decoding\n\n\\[16\\]
: enable for muti stream (vp7)\n\n\\[15:8\\]
: boolean dec init value(vp7)\n\n\\[7:0\\]
: boolean dec init range"]
    #[inline(always)]
    pub fn mfr_reg2(&self) -> MfrReg2R {
        MfrReg2R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - multi format reuse register2 except h264\n\nMPEG4:\n\n\\[31:26\\]
: Exact bit of stream start word\n\n\\[25\\]
: enable for sync markers\n\n\\[24\\]
: enable for Type 1 quantization\n\n\\[23:19\\]
: the offset of Qp filter\n\n\\[18:14\\]
: the offset of Qp filter for cr\n\n\\[0\\]
: filed_pic_flag exists in stream\n\nJPEG :\n\n\\[31:26\\]
: Exact bit of stream start word\n\n\\[25\\]
: enable for sync markers\n\n\\[12:11\\]
: total of Quantization tables\n\n\\[10:8\\]
: the sampling format for input pic\n\n\\[7\\]
: JPEG width\n\n\\[6\\]
: weather current strem buffer contain the end of a\n\nJPEG image\n\n\\[5:0\\]
: vlc table\n\nvp6:\n\n\\[23:18\\]
: start bit for ctrl stream (vp7)\n\n\\[17\\]
: enable for huffman decoding\n\n\\[16\\]
: enable for muti stream (vp7)\n\n\\[15:8\\]
: boolean dec init value(vp7)\n\n\\[7:0\\]
: boolean dec init range"]
    #[inline(always)]
    #[must_use]
    pub fn mfr_reg2(&mut self) -> MfrReg2W<Swreg122Spec> {
        MfrReg2W::new(self, 0)
    }
}
#[doc = "multi format reuse register2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg122::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg122::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg122Spec;
impl crate::RegisterSpec for Swreg122Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg122::R`](R) reader structure"]
impl crate::Readable for Swreg122Spec {}
#[doc = "`write(|w| ..)` method takes [`swreg122::W`](W) writer structure"]
impl crate::Writable for Swreg122Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG122 to value 0"]
impl crate::Resettable for Swreg122Spec {
    const RESET_VALUE: u32 = 0;
}
