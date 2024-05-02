#[doc = "Register `GAMMA_R_Y%s` reader"]
pub type R = crate::R<GammaRYSpec>;
#[doc = "Register `GAMMA_R_Y%s` writer"]
pub type W = crate::W<GammaRYSpec>;
#[doc = "Field `GAMMA_R_Y` reader - gamma curve point definition y-axis (output) for\n\nred RESTRICTION: each Y must be in the\n\n+2047/-2048 range compared to its predecessor (so\n\nthat the difference between successive Y values is\n\n12-bit signed !)"]
pub type GammaRYR = crate::FieldReader<u16>;
#[doc = "Field `GAMMA_R_Y` writer - gamma curve point definition y-axis (output) for\n\nred RESTRICTION: each Y must be in the\n\n+2047/-2048 range compared to its predecessor (so\n\nthat the difference between successive Y values is\n\n12-bit signed !)"]
pub type GammaRYW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - gamma curve point definition y-axis (output) for\n\nred RESTRICTION: each Y must be in the\n\n+2047/-2048 range compared to its predecessor (so\n\nthat the difference between successive Y values is\n\n12-bit signed !)"]
    #[inline(always)]
    pub fn gamma_r_y(&self) -> GammaRYR {
        GammaRYR::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - gamma curve point definition y-axis (output) for\n\nred RESTRICTION: each Y must be in the\n\n+2047/-2048 range compared to its predecessor (so\n\nthat the difference between successive Y values is\n\n12-bit signed !)"]
    #[inline(always)]
    #[must_use]
    pub fn gamma_r_y(&mut self) -> GammaRYW<GammaRYSpec> {
        GammaRYW::new(self, 0)
    }
}
#[doc = "De-Gamma Curve definition y red n (n=0..16)\n\nNote: The reset values define a linear curve which has the same effect as bypass. \n\n\n\nReset values are: Y_00 = 0x0000, Y_01 = 0x0100, Y_02 = 0x0200, Y_03 = 0x0300, Y_04 \n\n\n\n= 0x0400,Y_05 = 0x0500, Y_06 = 0x0600, Y_07 = 0x0700, Y_08 = 0x0800, Y_09 = \n\n\n\n0x0900, Y_10 = 0x0A00, Y_11 = 0x0B00, Y_12 = 0x0C00, Y_13 = 0x0D00, Y_14 = \n\n\n\n0x0E00, Y_15 = 0x0F00, Y_16 = 0x0FFF \n\n\n\n \n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gamma_r_y::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gamma_r_y::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GammaRYSpec;
impl crate::RegisterSpec for GammaRYSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gamma_r_y::R`](R) reader structure"]
impl crate::Readable for GammaRYSpec {}
#[doc = "`write(|w| ..)` method takes [`gamma_r_y::W`](W) writer structure"]
impl crate::Writable for GammaRYSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GAMMA_R_Y%s to value 0"]
impl crate::Resettable for GammaRYSpec {
    const RESET_VALUE: u32 = 0;
}
