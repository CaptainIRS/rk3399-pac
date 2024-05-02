#[doc = "Register `AFM_LT_B` reader"]
pub type R = crate::R<AfmLtBSpec>;
#[doc = "Register `AFM_LT_B` writer"]
pub type W = crate::W<AfmLtBSpec>;
#[doc = "Field `b_v_t` reader - first line of window B (vertical top line), value must\n\nbe greater or equal 2\n\n"]
pub type BVTR = crate::FieldReader<u16>;
#[doc = "Field `b_v_t` writer - first line of window B (vertical top line), value must\n\nbe greater or equal 2\n\n"]
pub type BVTW<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
#[doc = "Field `b_h_l` reader - first pixel of window B (horizontal left row),\n\nvalue must be greater or equal 5"]
pub type BHLR = crate::FieldReader<u16>;
#[doc = "Field `b_h_l` writer - first pixel of window B (horizontal left row),\n\nvalue must be greater or equal 5"]
pub type BHLW<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
impl R {
    #[doc = "Bits 0:12 - first line of window B (vertical top line), value must\n\nbe greater or equal 2\n\n"]
    #[inline(always)]
    pub fn b_v_t(&self) -> BVTR {
        BVTR::new((self.bits & 0x1fff) as u16)
    }
    #[doc = "Bits 16:28 - first pixel of window B (horizontal left row),\n\nvalue must be greater or equal 5"]
    #[inline(always)]
    pub fn b_h_l(&self) -> BHLR {
        BHLR::new(((self.bits >> 16) & 0x1fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:12 - first line of window B (vertical top line), value must\n\nbe greater or equal 2\n\n"]
    #[inline(always)]
    #[must_use]
    pub fn b_v_t(&mut self) -> BVTW<AfmLtBSpec> {
        BVTW::new(self, 0)
    }
    #[doc = "Bits 16:28 - first pixel of window B (horizontal left row),\n\nvalue must be greater or equal 5"]
    #[inline(always)]
    #[must_use]
    pub fn b_h_l(&mut self) -> BHLW<AfmLtBSpec> {
        BHLW::new(self, 16)
    }
}
#[doc = "Top left corner of measure window B\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`afm_lt_b::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`afm_lt_b::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AfmLtBSpec;
impl crate::RegisterSpec for AfmLtBSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`afm_lt_b::R`](R) reader structure"]
impl crate::Readable for AfmLtBSpec {}
#[doc = "`write(|w| ..)` method takes [`afm_lt_b::W`](W) writer structure"]
impl crate::Writable for AfmLtBSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AFM_LT_B to value 0"]
impl crate::Resettable for AfmLtBSpec {
    const RESET_VALUE: u32 = 0;
}
