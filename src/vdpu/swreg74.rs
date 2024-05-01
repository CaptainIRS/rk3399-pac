#[doc = "Register `SWREG74` reader"]
pub type R = crate::R<Swreg74Spec>;
#[doc = "Register `SWREG74` writer"]
pub type W = crate::W<Swreg74Spec>;
#[doc = "Field `H264_DIFF_MV_ST_ADR` reader - the Differential motion vector start address\n\nDifferential motion vector base address used for h264 only\n\nit also reuse used as:\n\n\\[29:25\\]
: 9st forward picid of inital reference pic list\n\n\\[24:20\\]
: 8st forward picid of inital reference pic list\n\n\\[19:15\\]
: 7st forward picid of inital reference pic list\n\n\\[14:10\\]
: 6st forward picid of inital reference pic list\n\n\\[9:5\\]
: 5st forward picid of inital reference pic list\n\n\\[4:0\\]
: 4st forward picid of inital reference pic list"]
pub type H264DiffMvStAdrR = crate::FieldReader<u32>;
#[doc = "Field `H264_DIFF_MV_ST_ADR` writer - the Differential motion vector start address\n\nDifferential motion vector base address used for h264 only\n\nit also reuse used as:\n\n\\[29:25\\]
: 9st forward picid of inital reference pic list\n\n\\[24:20\\]
: 8st forward picid of inital reference pic list\n\n\\[19:15\\]
: 7st forward picid of inital reference pic list\n\n\\[14:10\\]
: 6st forward picid of inital reference pic list\n\n\\[9:5\\]
: 5st forward picid of inital reference pic list\n\n\\[4:0\\]
: 4st forward picid of inital reference pic list"]
pub type H264DiffMvStAdrW<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    #[doc = "Bits 2:31 - the Differential motion vector start address\n\nDifferential motion vector base address used for h264 only\n\nit also reuse used as:\n\n\\[29:25\\]
: 9st forward picid of inital reference pic list\n\n\\[24:20\\]
: 8st forward picid of inital reference pic list\n\n\\[19:15\\]
: 7st forward picid of inital reference pic list\n\n\\[14:10\\]
: 6st forward picid of inital reference pic list\n\n\\[9:5\\]
: 5st forward picid of inital reference pic list\n\n\\[4:0\\]
: 4st forward picid of inital reference pic list"]
    #[inline(always)]
    pub fn h264_diff_mv_st_adr(&self) -> H264DiffMvStAdrR {
        H264DiffMvStAdrR::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bits 2:31 - the Differential motion vector start address\n\nDifferential motion vector base address used for h264 only\n\nit also reuse used as:\n\n\\[29:25\\]
: 9st forward picid of inital reference pic list\n\n\\[24:20\\]
: 8st forward picid of inital reference pic list\n\n\\[19:15\\]
: 7st forward picid of inital reference pic list\n\n\\[14:10\\]
: 6st forward picid of inital reference pic list\n\n\\[9:5\\]
: 5st forward picid of inital reference pic list\n\n\\[4:0\\]
: 4st forward picid of inital reference pic list"]
    #[inline(always)]
    #[must_use]
    pub fn h264_diff_mv_st_adr(&mut self) -> H264DiffMvStAdrW<Swreg74Spec> {
        H264DiffMvStAdrW::new(self, 2)
    }
}
#[doc = "MV address for h264\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg74::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg74::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg74Spec;
impl crate::RegisterSpec for Swreg74Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg74::R`](R) reader structure"]
impl crate::Readable for Swreg74Spec {}
#[doc = "`write(|w| ..)` method takes [`swreg74::W`](W) writer structure"]
impl crate::Writable for Swreg74Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG74 to value 0"]
impl crate::Resettable for Swreg74Spec {
    const RESET_VALUE: u32 = 0;
}
