#[doc = "Register `SWREG75` reader"]
pub type R = crate::R<Swreg75Spec>;
#[doc = "Register `SWREG75` writer"]
pub type W = crate::W<Swreg75Spec>;
#[doc = "Field `H264_PRED4X4_ST_ADR` reader - H.264 Intra prediction 4x4 mode start address\n\nalso be used as:\n\n\\[29:25\\]
: 15st forward picid of inital reference pic list\n\n\\[24:20\\]
: 14st forward picid of inital reference pic list\n\n\\[19:15\\]
: 13st forward picid of inital reference pic list\n\n\\[14:10\\]
: 12st forward picid of inital reference pic list\n\n\\[9:5\\]
: 11st forward picid of inital reference pic list\n\n\\[4:0\\]
: 10st forward picid of inital reference pic list"]
pub type H264Pred4x4StAdrR = crate::FieldReader<u32>;
#[doc = "Field `H264_PRED4X4_ST_ADR` writer - H.264 Intra prediction 4x4 mode start address\n\nalso be used as:\n\n\\[29:25\\]
: 15st forward picid of inital reference pic list\n\n\\[24:20\\]
: 14st forward picid of inital reference pic list\n\n\\[19:15\\]
: 13st forward picid of inital reference pic list\n\n\\[14:10\\]
: 12st forward picid of inital reference pic list\n\n\\[9:5\\]
: 11st forward picid of inital reference pic list\n\n\\[4:0\\]
: 10st forward picid of inital reference pic list"]
pub type H264Pred4x4StAdrW<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    #[doc = "Bits 2:31 - H.264 Intra prediction 4x4 mode start address\n\nalso be used as:\n\n\\[29:25\\]
: 15st forward picid of inital reference pic list\n\n\\[24:20\\]
: 14st forward picid of inital reference pic list\n\n\\[19:15\\]
: 13st forward picid of inital reference pic list\n\n\\[14:10\\]
: 12st forward picid of inital reference pic list\n\n\\[9:5\\]
: 11st forward picid of inital reference pic list\n\n\\[4:0\\]
: 10st forward picid of inital reference pic list"]
    #[inline(always)]
    pub fn h264_pred4x4_st_adr(&self) -> H264Pred4x4StAdrR {
        H264Pred4x4StAdrR::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bits 2:31 - H.264 Intra prediction 4x4 mode start address\n\nalso be used as:\n\n\\[29:25\\]
: 15st forward picid of inital reference pic list\n\n\\[24:20\\]
: 14st forward picid of inital reference pic list\n\n\\[19:15\\]
: 13st forward picid of inital reference pic list\n\n\\[14:10\\]
: 12st forward picid of inital reference pic list\n\n\\[9:5\\]
: 11st forward picid of inital reference pic list\n\n\\[4:0\\]
: 10st forward picid of inital reference pic list"]
    #[inline(always)]
    #[must_use]
    pub fn h264_pred4x4_st_adr(&mut self) -> H264Pred4x4StAdrW<Swreg75Spec> {
        H264Pred4x4StAdrW::new(self, 2)
    }
}
#[doc = "H.264 Intra prediction 4x4 mode start address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg75::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg75::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg75Spec;
impl crate::RegisterSpec for Swreg75Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg75::R`](R) reader structure"]
impl crate::Readable for Swreg75Spec {}
#[doc = "`write(|w| ..)` method takes [`swreg75::W`](W) writer structure"]
impl crate::Writable for Swreg75Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG75 to value 0"]
impl crate::Resettable for Swreg75Spec {
    const RESET_VALUE: u32 = 0;
}
