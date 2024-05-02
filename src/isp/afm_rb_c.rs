#[doc = "Register `AFM_RB_C` reader"]
pub type R = crate::R<AfmRbCSpec>;
#[doc = "Register `AFM_RB_C` writer"]
pub type W = crate::W<AfmRbCSpec>;
#[doc = "Field `c_v_b` reader - last line of window C (vertical bottom line), value\n\nmust be lower than (number of lines – 2)\n\n"]
pub type CVBR = crate::FieldReader<u16>;
#[doc = "Field `c_v_b` writer - last line of window C (vertical bottom line), value\n\nmust be lower than (number of lines – 2)\n\n"]
pub type CVBW<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
#[doc = "Field `c_h_r` reader - last pixel of window C (horizontal right row)"]
pub type CHRR = crate::FieldReader<u16>;
#[doc = "Field `c_h_r` writer - last pixel of window C (horizontal right row)"]
pub type CHRW<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
impl R {
    #[doc = "Bits 0:12 - last line of window C (vertical bottom line), value\n\nmust be lower than (number of lines – 2)\n\n"]
    #[inline(always)]
    pub fn c_v_b(&self) -> CVBR {
        CVBR::new((self.bits & 0x1fff) as u16)
    }
    #[doc = "Bits 16:28 - last pixel of window C (horizontal right row)"]
    #[inline(always)]
    pub fn c_h_r(&self) -> CHRR {
        CHRR::new(((self.bits >> 16) & 0x1fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:12 - last line of window C (vertical bottom line), value\n\nmust be lower than (number of lines – 2)\n\n"]
    #[inline(always)]
    #[must_use]
    pub fn c_v_b(&mut self) -> CVBW<AfmRbCSpec> {
        CVBW::new(self, 0)
    }
    #[doc = "Bits 16:28 - last pixel of window C (horizontal right row)"]
    #[inline(always)]
    #[must_use]
    pub fn c_h_r(&mut self) -> CHRW<AfmRbCSpec> {
        CHRW::new(self, 16)
    }
}
#[doc = "Bottom right corner of measure window C\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`afm_rb_c::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`afm_rb_c::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AfmRbCSpec;
impl crate::RegisterSpec for AfmRbCSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`afm_rb_c::R`](R) reader structure"]
impl crate::Readable for AfmRbCSpec {}
#[doc = "`write(|w| ..)` method takes [`afm_rb_c::W`](W) writer structure"]
impl crate::Writable for AfmRbCSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AFM_RB_C to value 0"]
impl crate::Resettable for AfmRbCSpec {
    const RESET_VALUE: u32 = 0;
}
