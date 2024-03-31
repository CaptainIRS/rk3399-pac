#[doc = "Register `CCI500_CON` reader"]
pub type R = crate::R<Cci500ConSpec>;
#[doc = "Register `CCI500_CON` writer"]
pub type W = crate::W<Cci500ConSpec>;
#[doc = "Field `PREQ_CCI500_CFG` reader - CCI-500 P-channel request sent by software"]
pub type PreqCci500CfgR = crate::BitReader;
#[doc = "Field `PREQ_CCI500_CFG` writer - CCI-500 P-channel request sent by software"]
pub type PreqCci500CfgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLR_PREQ_CCI500` reader - CCI-500 P-channel request sent by hardware ."]
pub type ClrPreqCci500R = crate::BitReader;
#[doc = "Field `CLR_PREQ_CCI500` writer - CCI-500 P-channel request sent by hardware ."]
pub type ClrPreqCci500W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PSTATE_CCI500` reader - CCI-500 P-channel pstate ."]
pub type PstateCci500R = crate::FieldReader;
#[doc = "Field `PSTATE_CCI500` writer - CCI-500 P-channel pstate ."]
pub type PstateCci500W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `QREQ_CCI500_CFG` reader - CCI-500 Q-channel request sent by software."]
pub type QreqCci500CfgR = crate::BitReader;
#[doc = "Field `QREQ_CCI500_CFG` writer - CCI-500 Q-channel request sent by software."]
pub type QreqCci500CfgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLR_QREQ_CCI500` reader - CCI-500 Q-channel request sent by hardware."]
pub type ClrQreqCci500R = crate::BitReader;
#[doc = "Field `CLR_QREQ_CCI500` writer - CCI-500 Q-channel request sent by hardware."]
pub type ClrQreqCci500W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `QGATING_CCI500_CFG` reader - CCI-500 Q-channel clock gating enable."]
pub type QgatingCci500CfgR = crate::BitReader;
#[doc = "Field `QGATING_CCI500_CFG` writer - CCI-500 Q-channel clock gating enable."]
pub type QgatingCci500CfgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WRITE_ENABLE` reader - When bit 16=1, bit 0 can be written by software .\n\nWhen bit 16=0, bit 0 cannot be written by software;\n\nWhen bit 17=1, bit 1 can be written by software .\n\nWhen bit 17=0, bit 1 cannot be written by software;\n\n......\n\nWhen bit 31=1, bit 15 can be written by software .\n\nWhen bit 31=0, bit 15 cannot be written by software;"]
pub type WriteEnableR = crate::FieldReader<u16>;
#[doc = "Field `WRITE_ENABLE` writer - When bit 16=1, bit 0 can be written by software .\n\nWhen bit 16=0, bit 0 cannot be written by software;\n\nWhen bit 17=1, bit 1 can be written by software .\n\nWhen bit 17=0, bit 1 cannot be written by software;\n\n......\n\nWhen bit 31=1, bit 15 can be written by software .\n\nWhen bit 31=0, bit 15 cannot be written by software;"]
pub type WriteEnableW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bit 0 - CCI-500 P-channel request sent by software"]
    #[inline(always)]
    pub fn preq_cci500_cfg(&self) -> PreqCci500CfgR {
        PreqCci500CfgR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CCI-500 P-channel request sent by hardware ."]
    #[inline(always)]
    pub fn clr_preq_cci500(&self) -> ClrPreqCci500R {
        ClrPreqCci500R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:4 - CCI-500 P-channel pstate ."]
    #[inline(always)]
    pub fn pstate_cci500(&self) -> PstateCci500R {
        PstateCci500R::new(((self.bits >> 2) & 7) as u8)
    }
    #[doc = "Bit 5 - CCI-500 Q-channel request sent by software."]
    #[inline(always)]
    pub fn qreq_cci500_cfg(&self) -> QreqCci500CfgR {
        QreqCci500CfgR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - CCI-500 Q-channel request sent by hardware."]
    #[inline(always)]
    pub fn clr_qreq_cci500(&self) -> ClrQreqCci500R {
        ClrQreqCci500R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - CCI-500 Q-channel clock gating enable."]
    #[inline(always)]
    pub fn qgating_cci500_cfg(&self) -> QgatingCci500CfgR {
        QgatingCci500CfgR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 16:31 - When bit 16=1, bit 0 can be written by software .\n\nWhen bit 16=0, bit 0 cannot be written by software;\n\nWhen bit 17=1, bit 1 can be written by software .\n\nWhen bit 17=0, bit 1 cannot be written by software;\n\n......\n\nWhen bit 31=1, bit 15 can be written by software .\n\nWhen bit 31=0, bit 15 cannot be written by software;"]
    #[inline(always)]
    pub fn write_enable(&self) -> WriteEnableR {
        WriteEnableR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - CCI-500 P-channel request sent by software"]
    #[inline(always)]
    #[must_use]
    pub fn preq_cci500_cfg(&mut self) -> PreqCci500CfgW<Cci500ConSpec> {
        PreqCci500CfgW::new(self, 0)
    }
    #[doc = "Bit 1 - CCI-500 P-channel request sent by hardware ."]
    #[inline(always)]
    #[must_use]
    pub fn clr_preq_cci500(&mut self) -> ClrPreqCci500W<Cci500ConSpec> {
        ClrPreqCci500W::new(self, 1)
    }
    #[doc = "Bits 2:4 - CCI-500 P-channel pstate ."]
    #[inline(always)]
    #[must_use]
    pub fn pstate_cci500(&mut self) -> PstateCci500W<Cci500ConSpec> {
        PstateCci500W::new(self, 2)
    }
    #[doc = "Bit 5 - CCI-500 Q-channel request sent by software."]
    #[inline(always)]
    #[must_use]
    pub fn qreq_cci500_cfg(&mut self) -> QreqCci500CfgW<Cci500ConSpec> {
        QreqCci500CfgW::new(self, 5)
    }
    #[doc = "Bit 6 - CCI-500 Q-channel request sent by hardware."]
    #[inline(always)]
    #[must_use]
    pub fn clr_qreq_cci500(&mut self) -> ClrQreqCci500W<Cci500ConSpec> {
        ClrQreqCci500W::new(self, 6)
    }
    #[doc = "Bit 7 - CCI-500 Q-channel clock gating enable."]
    #[inline(always)]
    #[must_use]
    pub fn qgating_cci500_cfg(&mut self) -> QgatingCci500CfgW<Cci500ConSpec> {
        QgatingCci500CfgW::new(self, 7)
    }
    #[doc = "Bits 16:31 - When bit 16=1, bit 0 can be written by software .\n\nWhen bit 16=0, bit 0 cannot be written by software;\n\nWhen bit 17=1, bit 1 can be written by software .\n\nWhen bit 17=0, bit 1 cannot be written by software;\n\n......\n\nWhen bit 31=1, bit 15 can be written by software .\n\nWhen bit 31=0, bit 15 cannot be written by software;"]
    #[inline(always)]
    #[must_use]
    pub fn write_enable(&mut self) -> WriteEnableW<Cci500ConSpec> {
        WriteEnableW::new(self, 16)
    }
}
#[doc = "CCI-500 low power control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cci500_con::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cci500_con::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cci500ConSpec;
impl crate::RegisterSpec for Cci500ConSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cci500_con::R`](R) reader structure"]
impl crate::Readable for Cci500ConSpec {}
#[doc = "`write(|w| ..)` method takes [`cci500_con::W`](W) writer structure"]
impl crate::Writable for Cci500ConSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CCI500_CON to value 0"]
impl crate::Resettable for Cci500ConSpec {
    const RESET_VALUE: u32 = 0;
}
