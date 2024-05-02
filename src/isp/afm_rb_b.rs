#[doc = "Register `AFM_RB_B` reader"]
pub type R = crate::R<AfmRbBSpec>;
#[doc = "Register `AFM_RB_B` writer"]
pub type W = crate::W<AfmRbBSpec>;
#[doc = "Field `b_v_b` reader - last line of window B (vertical bottom line), value\n\nmust be lower than (number of lines – 2)\n\n"]
pub type BVBR = crate::FieldReader<u16>;
#[doc = "Field `b_v_b` writer - last line of window B (vertical bottom line), value\n\nmust be lower than (number of lines – 2)\n\n"]
pub type BVBW<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
#[doc = "Field `b_h_r` reader - last pixel of window B (horizontal right row)"]
pub type BHRR = crate::FieldReader<u16>;
#[doc = "Field `b_h_r` writer - last pixel of window B (horizontal right row)"]
pub type BHRW<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
impl R {
    #[doc = "Bits 0:12 - last line of window B (vertical bottom line), value\n\nmust be lower than (number of lines – 2)\n\n"]
    #[inline(always)]
    pub fn b_v_b(&self) -> BVBR {
        BVBR::new((self.bits & 0x1fff) as u16)
    }
    #[doc = "Bits 16:28 - last pixel of window B (horizontal right row)"]
    #[inline(always)]
    pub fn b_h_r(&self) -> BHRR {
        BHRR::new(((self.bits >> 16) & 0x1fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:12 - last line of window B (vertical bottom line), value\n\nmust be lower than (number of lines – 2)\n\n"]
    #[inline(always)]
    #[must_use]
    pub fn b_v_b(&mut self) -> BVBW<AfmRbBSpec> {
        BVBW::new(self, 0)
    }
    #[doc = "Bits 16:28 - last pixel of window B (horizontal right row)"]
    #[inline(always)]
    #[must_use]
    pub fn b_h_r(&mut self) -> BHRW<AfmRbBSpec> {
        BHRW::new(self, 16)
    }
}
#[doc = "Bottom right corner of measure window B\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`afm_rb_b::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`afm_rb_b::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AfmRbBSpec;
impl crate::RegisterSpec for AfmRbBSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`afm_rb_b::R`](R) reader structure"]
impl crate::Readable for AfmRbBSpec {}
#[doc = "`write(|w| ..)` method takes [`afm_rb_b::W`](W) writer structure"]
impl crate::Writable for AfmRbBSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AFM_RB_B to value 0"]
impl crate::Resettable for AfmRbBSpec {
    const RESET_VALUE: u32 = 0;
}
