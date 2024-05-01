#[doc = "Register `SWREG_104` reader"]
pub type R = crate::R<Swreg104Spec>;
#[doc = "Register `SWREG_104` writer"]
pub type W = crate::W<Swreg104Spec>;
#[doc = "Field `MB_CNT` reader - macroblock_count\n\nmacroblock_count"]
pub type MbCntR = crate::FieldReader<u16>;
#[doc = "Field `MB_CNT` writer - macroblock_count\n\nmacroblock_count"]
pub type MbCntW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `MB_COUNT_OUT` reader - mb_count_out\n\nmb_count_out"]
pub type MbCountOutR = crate::FieldReader<u16>;
#[doc = "Field `MB_COUNT_OUT` writer - mb_count_out\n\nmb_count_out"]
pub type MbCountOutW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - macroblock_count\n\nmacroblock_count"]
    #[inline(always)]
    pub fn mb_cnt(&self) -> MbCntR {
        MbCntR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - mb_count_out\n\nmb_count_out"]
    #[inline(always)]
    pub fn mb_count_out(&self) -> MbCountOutR {
        MbCountOutR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - macroblock_count\n\nmacroblock_count"]
    #[inline(always)]
    #[must_use]
    pub fn mb_cnt(&mut self) -> MbCntW<Swreg104Spec> {
        MbCntW::new(self, 0)
    }
    #[doc = "Bits 16:31 - mb_count_out\n\nmb_count_out"]
    #[inline(always)]
    #[must_use]
    pub fn mb_count_out(&mut self) -> MbCountOutW<Swreg104Spec> {
        MbCountOutW::new(self, 16)
    }
}
#[doc = "mb control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg_104::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg_104::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg104Spec;
impl crate::RegisterSpec for Swreg104Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg_104::R`](R) reader structure"]
impl crate::Readable for Swreg104Spec {}
#[doc = "`write(|w| ..)` method takes [`swreg_104::W`](W) writer structure"]
impl crate::Writable for Swreg104Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG_104 to value 0"]
impl crate::Resettable for Swreg104Spec {
    const RESET_VALUE: u32 = 0;
}
