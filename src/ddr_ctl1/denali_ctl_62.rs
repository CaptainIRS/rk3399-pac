#[doc = "Register `DENALI_CTL_62` reader"]
pub type R = crate::R<DenaliCtl62Spec>;
#[doc = "Register `DENALI_CTL_62` writer"]
pub type W = crate::W<DenaliCtl62Spec>;
#[doc = "Field `TXSNR_F2` reader - DRAM TXSNR value for frequency copy 2 in cycles."]
pub type TxsnrF2R = crate::FieldReader<u16>;
#[doc = "Field `TXSNR_F2` writer - DRAM TXSNR value for frequency copy 2 in cycles."]
pub type TxsnrF2W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `TCKELCMD_F0` reader - DRAM TCKELCMD value for frequency copy 0 in cycles."]
pub type TckelcmdF0R = crate::FieldReader;
#[doc = "Field `TCKELCMD_F0` writer - DRAM TCKELCMD value for frequency copy 0 in cycles."]
pub type TckelcmdF0W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `TCKEHCMD_F0` reader - DRAM TCKEHCMD value for frequency copy 0 in cycles."]
pub type TckehcmdF0R = crate::FieldReader;
#[doc = "Field `TCKEHCMD_F0` writer - DRAM TCKEHCMD value for frequency copy 0 in cycles."]
pub type TckehcmdF0W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:15 - DRAM TXSNR value for frequency copy 2 in cycles."]
    #[inline(always)]
    pub fn txsnr_f2(&self) -> TxsnrF2R {
        TxsnrF2R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:19 - DRAM TCKELCMD value for frequency copy 0 in cycles."]
    #[inline(always)]
    pub fn tckelcmd_f0(&self) -> TckelcmdF0R {
        TckelcmdF0R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - DRAM TCKEHCMD value for frequency copy 0 in cycles."]
    #[inline(always)]
    pub fn tckehcmd_f0(&self) -> TckehcmdF0R {
        TckehcmdF0R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:15 - DRAM TXSNR value for frequency copy 2 in cycles."]
    #[inline(always)]
    #[must_use]
    pub fn txsnr_f2(&mut self) -> TxsnrF2W<DenaliCtl62Spec> {
        TxsnrF2W::new(self, 0)
    }
    #[doc = "Bits 16:19 - DRAM TCKELCMD value for frequency copy 0 in cycles."]
    #[inline(always)]
    #[must_use]
    pub fn tckelcmd_f0(&mut self) -> TckelcmdF0W<DenaliCtl62Spec> {
        TckelcmdF0W::new(self, 16)
    }
    #[doc = "Bits 24:27 - DRAM TCKEHCMD value for frequency copy 0 in cycles."]
    #[inline(always)]
    #[must_use]
    pub fn tckehcmd_f0(&mut self) -> TckehcmdF0W<DenaliCtl62Spec> {
        TckehcmdF0W::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_62::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_62::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliCtl62Spec;
impl crate::RegisterSpec for DenaliCtl62Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_ctl_62::R`](R) reader structure"]
impl crate::Readable for DenaliCtl62Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_ctl_62::W`](W) writer structure"]
impl crate::Writable for DenaliCtl62Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_CTL_62 to value 0"]
impl crate::Resettable for DenaliCtl62Spec {
    const RESET_VALUE: u32 = 0;
}
