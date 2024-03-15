#[doc = "Register `PI_REG_118` reader"]
pub type R = crate::R<PiReg118Spec>;
#[doc = "Register `PI_REG_118` writer"]
pub type W = crate::W<PiReg118Spec>;
#[doc = "Field `PI_TDFI_WDQLVL_WR` reader - Switches time from write to read."]
pub type PiTdfiWdqlvlWrR = crate::FieldReader<u16>;
#[doc = "Field `PI_TDFI_WDQLVL_WR` writer - Switches time from write to read."]
pub type PiTdfiWdqlvlWrW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `PI_TDFI_WDQLVL_RW` reader - Switches time from read to write."]
pub type PiTdfiWdqlvlRwR = crate::FieldReader<u16>;
#[doc = "Field `PI_TDFI_WDQLVL_RW` writer - Switches time from read to write."]
pub type PiTdfiWdqlvlRwW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - Switches time from write to read."]
    #[inline(always)]
    pub fn pi_tdfi_wdqlvl_wr(&self) -> PiTdfiWdqlvlWrR {
        PiTdfiWdqlvlWrR::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 16:25 - Switches time from read to write."]
    #[inline(always)]
    pub fn pi_tdfi_wdqlvl_rw(&self) -> PiTdfiWdqlvlRwR {
        PiTdfiWdqlvlRwR::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - Switches time from write to read."]
    #[inline(always)]
    #[must_use]
    pub fn pi_tdfi_wdqlvl_wr(&mut self) -> PiTdfiWdqlvlWrW<PiReg118Spec> {
        PiTdfiWdqlvlWrW::new(self, 0)
    }
    #[doc = "Bits 16:25 - Switches time from read to write."]
    #[inline(always)]
    #[must_use]
    pub fn pi_tdfi_wdqlvl_rw(&mut self) -> PiTdfiWdqlvlRwW<PiReg118Spec> {
        PiTdfiWdqlvlRwW::new(self, 16)
    }
}
#[doc = "DDR PHY Independent Register 118\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pi_reg_118::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pi_reg_118::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PiReg118Spec;
impl crate::RegisterSpec for PiReg118Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pi_reg_118::R`](R) reader structure"]
impl crate::Readable for PiReg118Spec {}
#[doc = "`write(|w| ..)` method takes [`pi_reg_118::W`](W) writer structure"]
impl crate::Writable for PiReg118Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PI_REG_118 to value 0"]
impl crate::Resettable for PiReg118Spec {
    const RESET_VALUE: u32 = 0;
}
