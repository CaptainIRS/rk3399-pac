#[doc = "Register `DENALI_CTL_66` reader"]
pub type R = crate::R<DenaliCtl66Spec>;
#[doc = "Register `DENALI_CTL_66` writer"]
pub type W = crate::W<DenaliCtl66Spec>;
#[doc = "Field `TCSCKEH_F1` reader - DRAM TCSCKEH value for frequency copy 1 in cycles."]
pub type TcsckehF1R = crate::FieldReader;
#[doc = "Field `TCSCKEH_F1` writer - DRAM TCSCKEH value for frequency copy 1 in cycles."]
pub type TcsckehF1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `TCMDCKE_F1` reader - DRAM TCMDCKE value for frequency copy 1 in cycles."]
pub type TcmdckeF1R = crate::FieldReader;
#[doc = "Field `TCMDCKE_F1` writer - DRAM TCMDCKE value for frequency copy 1 in cycles."]
pub type TcmdckeF1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `TCKELCMD_F2` reader - DRAM TCKELCMD value for frequency copy 2 in cycles."]
pub type TckelcmdF2R = crate::FieldReader;
#[doc = "Field `TCKELCMD_F2` writer - DRAM TCKELCMD value for frequency copy 2 in cycles."]
pub type TckelcmdF2W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `TCKEHCMD_F2` reader - DRAM TCKEHCMD value for frequency copy 2 in cycles."]
pub type TckehcmdF2R = crate::FieldReader;
#[doc = "Field `TCKEHCMD_F2` writer - DRAM TCKEHCMD value for frequency copy 2 in cycles."]
pub type TckehcmdF2W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - DRAM TCSCKEH value for frequency copy 1 in cycles."]
    #[inline(always)]
    pub fn tcsckeh_f1(&self) -> TcsckehF1R {
        TcsckehF1R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - DRAM TCMDCKE value for frequency copy 1 in cycles."]
    #[inline(always)]
    pub fn tcmdcke_f1(&self) -> TcmdckeF1R {
        TcmdckeF1R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - DRAM TCKELCMD value for frequency copy 2 in cycles."]
    #[inline(always)]
    pub fn tckelcmd_f2(&self) -> TckelcmdF2R {
        TckelcmdF2R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - DRAM TCKEHCMD value for frequency copy 2 in cycles."]
    #[inline(always)]
    pub fn tckehcmd_f2(&self) -> TckehcmdF2R {
        TckehcmdF2R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - DRAM TCSCKEH value for frequency copy 1 in cycles."]
    #[inline(always)]
    #[must_use]
    pub fn tcsckeh_f1(&mut self) -> TcsckehF1W<DenaliCtl66Spec> {
        TcsckehF1W::new(self, 0)
    }
    #[doc = "Bits 8:11 - DRAM TCMDCKE value for frequency copy 1 in cycles."]
    #[inline(always)]
    #[must_use]
    pub fn tcmdcke_f1(&mut self) -> TcmdckeF1W<DenaliCtl66Spec> {
        TcmdckeF1W::new(self, 8)
    }
    #[doc = "Bits 16:19 - DRAM TCKELCMD value for frequency copy 2 in cycles."]
    #[inline(always)]
    #[must_use]
    pub fn tckelcmd_f2(&mut self) -> TckelcmdF2W<DenaliCtl66Spec> {
        TckelcmdF2W::new(self, 16)
    }
    #[doc = "Bits 24:27 - DRAM TCKEHCMD value for frequency copy 2 in cycles."]
    #[inline(always)]
    #[must_use]
    pub fn tckehcmd_f2(&mut self) -> TckehcmdF2W<DenaliCtl66Spec> {
        TckehcmdF2W::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_66::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_66::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliCtl66Spec;
impl crate::RegisterSpec for DenaliCtl66Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_ctl_66::R`](R) reader structure"]
impl crate::Readable for DenaliCtl66Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_ctl_66::W`](W) writer structure"]
impl crate::Writable for DenaliCtl66Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_CTL_66 to value 0"]
impl crate::Resettable for DenaliCtl66Spec {
    const RESET_VALUE: u32 = 0;
}
