#[doc = "Register `DDR_DENALI_CTL_64` reader"]
pub type R = crate::R<DdrDenaliCtl64Spec>;
#[doc = "Register `DDR_DENALI_CTL_64` writer"]
pub type W = crate::W<DdrDenaliCtl64Spec>;
#[doc = "Field `TCSCKEH_F0` reader - DRAM TCSCKEH value for frequency copy 0 in cycles."]
pub type TcsckehF0R = crate::FieldReader;
#[doc = "Field `TCSCKEH_F0` writer - DRAM TCSCKEH value for frequency copy 0 in cycles."]
pub type TcsckehF0W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `TCMDCKE_F0` reader - DRAM TCMDCKE value for frequency copy 0 in cycles."]
pub type TcmdckeF0R = crate::FieldReader;
#[doc = "Field `TCMDCKE_F0` writer - DRAM TCMDCKE value for frequency copy 0 in cycles."]
pub type TcmdckeF0W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `TCKELCMD_F1` reader - DRAM TCKELCMD value for frequency copy 1 in cycles."]
pub type TckelcmdF1R = crate::FieldReader;
#[doc = "Field `TCKELCMD_F1` writer - DRAM TCKELCMD value for frequency copy 1 in cycles."]
pub type TckelcmdF1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `TCKEHCMD_F1` reader - DRAM TCKEHCMD value for frequency copy 1 in cycles."]
pub type TckehcmdF1R = crate::FieldReader;
#[doc = "Field `TCKEHCMD_F1` writer - DRAM TCKEHCMD value for frequency copy 1 in cycles."]
pub type TckehcmdF1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - DRAM TCSCKEH value for frequency copy 0 in cycles."]
    #[inline(always)]
    pub fn tcsckeh_f0(&self) -> TcsckehF0R {
        TcsckehF0R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - DRAM TCMDCKE value for frequency copy 0 in cycles."]
    #[inline(always)]
    pub fn tcmdcke_f0(&self) -> TcmdckeF0R {
        TcmdckeF0R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - DRAM TCKELCMD value for frequency copy 1 in cycles."]
    #[inline(always)]
    pub fn tckelcmd_f1(&self) -> TckelcmdF1R {
        TckelcmdF1R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - DRAM TCKEHCMD value for frequency copy 1 in cycles."]
    #[inline(always)]
    pub fn tckehcmd_f1(&self) -> TckehcmdF1R {
        TckehcmdF1R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - DRAM TCSCKEH value for frequency copy 0 in cycles."]
    #[inline(always)]
    #[must_use]
    pub fn tcsckeh_f0(&mut self) -> TcsckehF0W<DdrDenaliCtl64Spec> {
        TcsckehF0W::new(self, 0)
    }
    #[doc = "Bits 8:11 - DRAM TCMDCKE value for frequency copy 0 in cycles."]
    #[inline(always)]
    #[must_use]
    pub fn tcmdcke_f0(&mut self) -> TcmdckeF0W<DdrDenaliCtl64Spec> {
        TcmdckeF0W::new(self, 8)
    }
    #[doc = "Bits 16:19 - DRAM TCKELCMD value for frequency copy 1 in cycles."]
    #[inline(always)]
    #[must_use]
    pub fn tckelcmd_f1(&mut self) -> TckelcmdF1W<DdrDenaliCtl64Spec> {
        TckelcmdF1W::new(self, 16)
    }
    #[doc = "Bits 24:27 - DRAM TCKEHCMD value for frequency copy 1 in cycles."]
    #[inline(always)]
    #[must_use]
    pub fn tckehcmd_f1(&mut self) -> TckehcmdF1W<DdrDenaliCtl64Spec> {
        TckehcmdF1W::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_ctl_64::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_denali_ctl_64::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliCtl64Spec;
impl crate::RegisterSpec for DdrDenaliCtl64Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_ctl_64::R`](R) reader structure"]
impl crate::Readable for DdrDenaliCtl64Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_denali_ctl_64::W`](W) writer structure"]
impl crate::Writable for DdrDenaliCtl64Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_DENALI_CTL_64 to value 0"]
impl crate::Resettable for DdrDenaliCtl64Spec {
    const RESET_VALUE: u32 = 0;
}
