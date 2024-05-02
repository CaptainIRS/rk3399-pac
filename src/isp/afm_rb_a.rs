#[doc = "Register `AFM_RB_A` reader"]
pub type R = crate::R<AfmRbASpec>;
#[doc = "Register `AFM_RB_A` writer"]
pub type W = crate::W<AfmRbASpec>;
#[doc = "Field `a_v_b` reader - last line of window A (vertical bottom line), value must\n\nbe lower than (number of lines – 2)"]
pub type AVBR = crate::FieldReader<u16>;
#[doc = "Field `a_v_b` writer - last line of window A (vertical bottom line), value must\n\nbe lower than (number of lines – 2)"]
pub type AVBW<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
#[doc = "Field `a_h_r` reader - last pixel of window A (horizontal right row)"]
pub type AHRR = crate::FieldReader<u16>;
#[doc = "Field `a_h_r` writer - last pixel of window A (horizontal right row)"]
pub type AHRW<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
impl R {
    #[doc = "Bits 0:12 - last line of window A (vertical bottom line), value must\n\nbe lower than (number of lines – 2)"]
    #[inline(always)]
    pub fn a_v_b(&self) -> AVBR {
        AVBR::new((self.bits & 0x1fff) as u16)
    }
    #[doc = "Bits 16:28 - last pixel of window A (horizontal right row)"]
    #[inline(always)]
    pub fn a_h_r(&self) -> AHRR {
        AHRR::new(((self.bits >> 16) & 0x1fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:12 - last line of window A (vertical bottom line), value must\n\nbe lower than (number of lines – 2)"]
    #[inline(always)]
    #[must_use]
    pub fn a_v_b(&mut self) -> AVBW<AfmRbASpec> {
        AVBW::new(self, 0)
    }
    #[doc = "Bits 16:28 - last pixel of window A (horizontal right row)"]
    #[inline(always)]
    #[must_use]
    pub fn a_h_r(&mut self) -> AHRW<AfmRbASpec> {
        AHRW::new(self, 16)
    }
}
#[doc = "Bottom right corner of measure window A\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`afm_rb_a::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`afm_rb_a::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AfmRbASpec;
impl crate::RegisterSpec for AfmRbASpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`afm_rb_a::R`](R) reader structure"]
impl crate::Readable for AfmRbASpec {}
#[doc = "`write(|w| ..)` method takes [`afm_rb_a::W`](W) writer structure"]
impl crate::Writable for AfmRbASpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AFM_RB_A to value 0"]
impl crate::Resettable for AfmRbASpec {
    const RESET_VALUE: u32 = 0;
}
