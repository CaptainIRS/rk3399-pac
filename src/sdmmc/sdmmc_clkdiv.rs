#[doc = "Register `SDMMC_CLKDIV` reader"]
pub type R = crate::R<SdmmcClkdivSpec>;
#[doc = "Register `SDMMC_CLKDIV` writer"]
pub type W = crate::W<SdmmcClkdivSpec>;
#[doc = "Field `CLK_DIVIDER0` reader - Clock divider-0 value. Clock division is 2*n.\n\nFor example, value of 0 means divide by 2*0 = 0 (no division,\n\nbypass), value of 1 means divide by 2*1 = 2, value of \"ff\"means\n\ndivide by 2*255 = 510, and so on"]
pub type ClkDivider0R = crate::FieldReader;
#[doc = "Field `CLK_DIVIDER0` writer - Clock divider-0 value. Clock division is 2*n.\n\nFor example, value of 0 means divide by 2*0 = 0 (no division,\n\nbypass), value of 1 means divide by 2*1 = 2, value of \"ff\"means\n\ndivide by 2*255 = 510, and so on"]
pub type ClkDivider0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Clock divider-0 value. Clock division is 2*n.\n\nFor example, value of 0 means divide by 2*0 = 0 (no division,\n\nbypass), value of 1 means divide by 2*1 = 2, value of \"ff\"means\n\ndivide by 2*255 = 510, and so on"]
    #[inline(always)]
    pub fn clk_divider0(&self) -> ClkDivider0R {
        ClkDivider0R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Clock divider-0 value. Clock division is 2*n.\n\nFor example, value of 0 means divide by 2*0 = 0 (no division,\n\nbypass), value of 1 means divide by 2*1 = 2, value of \"ff\"means\n\ndivide by 2*255 = 510, and so on"]
    #[inline(always)]
    #[must_use]
    pub fn clk_divider0(&mut self) -> ClkDivider0W<SdmmcClkdivSpec> {
        ClkDivider0W::new(self, 0)
    }
}
#[doc = "Clock-divider register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdmmc_clkdiv::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdmmc_clkdiv::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SdmmcClkdivSpec;
impl crate::RegisterSpec for SdmmcClkdivSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sdmmc_clkdiv::R`](R) reader structure"]
impl crate::Readable for SdmmcClkdivSpec {}
#[doc = "`write(|w| ..)` method takes [`sdmmc_clkdiv::W`](W) writer structure"]
impl crate::Writable for SdmmcClkdivSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SDMMC_CLKDIV to value 0"]
impl crate::Resettable for SdmmcClkdivSpec {
    const RESET_VALUE: u32 = 0;
}
