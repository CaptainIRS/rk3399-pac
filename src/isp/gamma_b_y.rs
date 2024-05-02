#[doc = "Register `GAMMA_B_Y%s` reader"]
pub type R = crate::R<GammaBYSpec>;
#[doc = "Register `GAMMA_B_Y%s` writer"]
pub type W = crate::W<GammaBYSpec>;
#[doc = "Field `GAMMA_B_Y` reader - gamma curve point definition y-axis (output) for blue\n\nRESTRICTION: each Y must be in the +2047/-2048 range\n\ncompared to its predecessor (so that the difference\n\nbetween successive Y values is 12-bit signed !)"]
pub type GammaBYR = crate::FieldReader<u16>;
#[doc = "Field `GAMMA_B_Y` writer - gamma curve point definition y-axis (output) for blue\n\nRESTRICTION: each Y must be in the +2047/-2048 range\n\ncompared to its predecessor (so that the difference\n\nbetween successive Y values is 12-bit signed !)"]
pub type GammaBYW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - gamma curve point definition y-axis (output) for blue\n\nRESTRICTION: each Y must be in the +2047/-2048 range\n\ncompared to its predecessor (so that the difference\n\nbetween successive Y values is 12-bit signed !)"]
    #[inline(always)]
    pub fn gamma_b_y(&self) -> GammaBYR {
        GammaBYR::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - gamma curve point definition y-axis (output) for blue\n\nRESTRICTION: each Y must be in the +2047/-2048 range\n\ncompared to its predecessor (so that the difference\n\nbetween successive Y values is 12-bit signed !)"]
    #[inline(always)]
    #[must_use]
    pub fn gamma_b_y(&mut self) -> GammaBYW<GammaBYSpec> {
        GammaBYW::new(self, 0)
    }
}
#[doc = "De-Gamma Curve definition y blue n (n=0..16)\n\nNote: The reset values define a linear curve which has the same effect as bypass. \n\n\n\nReset values are: Y_00 = 0x0000, Y_01 = 0x0100, Y_02 = 0x0200, Y_03 = 0x0300, Y_04 \n\n\n\n= 0x0400, \n\n\n\nY_05 = 0x0500, Y_06 = 0x0600, Y_07 = 0x0700, Y_08 = 0x0800, \n\n\n\nY_09 = 0x0900, Y_10 = 0x0A00, Y_11 = 0x0B00, Y_12 = 0x0C00, Y_13 = \n\n\n\n0x0D00, Y_14 = 0x0E00, Y_15 = 0x0F00, Y_16 = 0x0FFF \n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gamma_b_y::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gamma_b_y::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GammaBYSpec;
impl crate::RegisterSpec for GammaBYSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gamma_b_y::R`](R) reader structure"]
impl crate::Readable for GammaBYSpec {}
#[doc = "`write(|w| ..)` method takes [`gamma_b_y::W`](W) writer structure"]
impl crate::Writable for GammaBYSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GAMMA_B_Y%s to value 0"]
impl crate::Resettable for GammaBYSpec {
    const RESET_VALUE: u32 = 0;
}
