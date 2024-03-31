#[doc = "Register `SOFTRST_CON1` reader"]
pub type R = crate::R<SoftrstCon1Spec>;
#[doc = "Register `SOFTRST_CON1` writer"]
pub type W = crate::W<SoftrstCon1Spec>;
#[doc = "Field `CORE0_L_SRSTN_REQ_T` reader - core0_l_srstn request bit\n\nWhen HIGH, reset relative logic"]
pub type Core0LSrstnReqTR = crate::BitReader;
#[doc = "Field `CORE0_L_SRSTN_REQ_T` writer - core0_l_srstn request bit\n\nWhen HIGH, reset relative logic"]
pub type Core0LSrstnReqTW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CORE1_L_SRSTN_REQ` reader - core1_l_srstn request bit\n\nWhen HIGH, reset relative logic"]
pub type Core1LSrstnReqR = crate::BitReader;
#[doc = "Field `CORE1_L_SRSTN_REQ` writer - core1_l_srstn request bit\n\nWhen HIGH, reset relative logic"]
pub type Core1LSrstnReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CORE2_L_SRSTN_REQ` reader - core2_l_srstn request bit\n\nWhen HIGH, reset relative logic"]
pub type Core2LSrstnReqR = crate::BitReader;
#[doc = "Field `CORE2_L_SRSTN_REQ` writer - core2_l_srstn request bit\n\nWhen HIGH, reset relative logic"]
pub type Core2LSrstnReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CORE3_L_SRSTN_REQ` reader - core3_l_srstn request bit\n\nWhen HIGH, reset relative logic"]
pub type Core3LSrstnReqR = crate::BitReader;
#[doc = "Field `CORE3_L_SRSTN_REQ` writer - core3_l_srstn request bit\n\nWhen HIGH, reset relative logic"]
pub type Core3LSrstnReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COREPO0_L_SRSTN_REQ_T` reader - corepo0_l_srstn request bit\n\nWhen HIGH, reset relative logic"]
pub type Corepo0LSrstnReqTR = crate::BitReader;
#[doc = "Field `COREPO0_L_SRSTN_REQ_T` writer - corepo0_l_srstn request bit\n\nWhen HIGH, reset relative logic"]
pub type Corepo0LSrstnReqTW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COREPO1_L_SRSTN_REQ` reader - corepo1_l_srstn request bit\n\nWhen HIGH, reset relative logic"]
pub type Corepo1LSrstnReqR = crate::BitReader;
#[doc = "Field `COREPO1_L_SRSTN_REQ` writer - corepo1_l_srstn request bit\n\nWhen HIGH, reset relative logic"]
pub type Corepo1LSrstnReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COREPO2_L_SRSTN_REQ` reader - corepo2_l_srstn request bit\n\nWhen HIGH, reset relative logic"]
pub type Corepo2LSrstnReqR = crate::BitReader;
#[doc = "Field `COREPO2_L_SRSTN_REQ` writer - corepo2_l_srstn request bit\n\nWhen HIGH, reset relative logic"]
pub type Corepo2LSrstnReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COREPO3_L_SRSTN_REQ` reader - corepo3_l_srstn request bit\n\nWhen HIGH, reset relative logic"]
pub type Corepo3LSrstnReqR = crate::BitReader;
#[doc = "Field `COREPO3_L_SRSTN_REQ` writer - corepo3_l_srstn request bit\n\nWhen HIGH, reset relative logic"]
pub type Corepo3LSrstnReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ARSTN_ADB400_GIC2COREL_REQ` reader - arstn_adb400_gic2corel request bit\n\nWhen HIGH, reset relative logic"]
pub type ArstnAdb400Gic2corelReqR = crate::BitReader;
#[doc = "Field `ARSTN_ADB400_GIC2COREL_REQ` writer - arstn_adb400_gic2corel request bit\n\nWhen HIGH, reset relative logic"]
pub type ArstnAdb400Gic2corelReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ARSTN_ADB400_COREL2GIC_REQ` reader - arstn_adb400_corel2gic request bit\n\nWhen HIGH, reset relative logic"]
pub type ArstnAdb400Corel2gicReqR = crate::BitReader;
#[doc = "Field `ARSTN_ADB400_COREL2GIC_REQ` writer - arstn_adb400_corel2gic request bit\n\nWhen HIGH, reset relative logic"]
pub type ArstnAdb400Corel2gicReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRSTN_DBG_L_REQ` reader - prstn_dbg_l request bit\n\nWhen HIGH, reset relative logic"]
pub type PrstnDbgLReqR = crate::BitReader;
#[doc = "Field `PRSTN_DBG_L_REQ` writer - prstn_dbg_l request bit\n\nWhen HIGH, reset relative logic"]
pub type PrstnDbgLReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `L2_L_SRSTN_REQ_T` reader - l2_l_srstn request bit\n\nWhen HIGH, reset relative logic"]
pub type L2LSrstnReqTR = crate::BitReader;
#[doc = "Field `L2_L_SRSTN_REQ_T` writer - l2_l_srstn request bit\n\nWhen HIGH, reset relative logic"]
pub type L2LSrstnReqTW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADB_L_SRSTN_REQ_T` reader - adb_l_srstn request bit\n\nWhen HIGH, reset relative logic"]
pub type AdbLSrstnReqTR = crate::BitReader;
#[doc = "Field `ADB_L_SRSTN_REQ_T` writer - adb_l_srstn request bit\n\nWhen HIGH, reset relative logic"]
pub type AdbLSrstnReqTW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RKPERF_L_ARSTN_REQ` reader - rkperf_l_arstn request bit\n\nWhen HIGH, reset relative logic"]
pub type RkperfLArstnReqR = crate::BitReader;
#[doc = "Field `RKPERF_L_ARSTN_REQ` writer - rkperf_l_arstn request bit\n\nWhen HIGH, reset relative logic"]
pub type RkperfLArstnReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PVTM_CORE_L_SRSTN_REQ` reader - pvtm_core_l_srstn request bit\n\nWhen HIGH, reset relative logic"]
pub type PvtmCoreLSrstnReqR = crate::BitReader;
#[doc = "Field `PVTM_CORE_L_SRSTN_REQ` writer - pvtm_core_l_srstn request bit\n\nWhen HIGH, reset relative logic"]
pub type PvtmCoreLSrstnReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WRITE_MASK` writer - write mask bits\n\nWhen every bit HIGH, enable the writing corresponding bit\n\nWhen every bit LOW, don't care the writing corresponding bit"]
pub type WriteMaskW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bit 0 - core0_l_srstn request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    pub fn core0_l_srstn_req_t(&self) -> Core0LSrstnReqTR {
        Core0LSrstnReqTR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - core1_l_srstn request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    pub fn core1_l_srstn_req(&self) -> Core1LSrstnReqR {
        Core1LSrstnReqR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - core2_l_srstn request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    pub fn core2_l_srstn_req(&self) -> Core2LSrstnReqR {
        Core2LSrstnReqR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - core3_l_srstn request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    pub fn core3_l_srstn_req(&self) -> Core3LSrstnReqR {
        Core3LSrstnReqR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - corepo0_l_srstn request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    pub fn corepo0_l_srstn_req_t(&self) -> Corepo0LSrstnReqTR {
        Corepo0LSrstnReqTR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - corepo1_l_srstn request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    pub fn corepo1_l_srstn_req(&self) -> Corepo1LSrstnReqR {
        Corepo1LSrstnReqR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - corepo2_l_srstn request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    pub fn corepo2_l_srstn_req(&self) -> Corepo2LSrstnReqR {
        Corepo2LSrstnReqR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - corepo3_l_srstn request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    pub fn corepo3_l_srstn_req(&self) -> Corepo3LSrstnReqR {
        Corepo3LSrstnReqR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - arstn_adb400_gic2corel request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    pub fn arstn_adb400_gic2corel_req(&self) -> ArstnAdb400Gic2corelReqR {
        ArstnAdb400Gic2corelReqR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - arstn_adb400_corel2gic request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    pub fn arstn_adb400_corel2gic_req(&self) -> ArstnAdb400Corel2gicReqR {
        ArstnAdb400Corel2gicReqR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - prstn_dbg_l request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    pub fn prstn_dbg_l_req(&self) -> PrstnDbgLReqR {
        PrstnDbgLReqR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 12 - l2_l_srstn request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    pub fn l2_l_srstn_req_t(&self) -> L2LSrstnReqTR {
        L2LSrstnReqTR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - adb_l_srstn request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    pub fn adb_l_srstn_req_t(&self) -> AdbLSrstnReqTR {
        AdbLSrstnReqTR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - rkperf_l_arstn request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    pub fn rkperf_l_arstn_req(&self) -> RkperfLArstnReqR {
        RkperfLArstnReqR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - pvtm_core_l_srstn request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    pub fn pvtm_core_l_srstn_req(&self) -> PvtmCoreLSrstnReqR {
        PvtmCoreLSrstnReqR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - core0_l_srstn request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    #[must_use]
    pub fn core0_l_srstn_req_t(&mut self) -> Core0LSrstnReqTW<SoftrstCon1Spec> {
        Core0LSrstnReqTW::new(self, 0)
    }
    #[doc = "Bit 1 - core1_l_srstn request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    #[must_use]
    pub fn core1_l_srstn_req(&mut self) -> Core1LSrstnReqW<SoftrstCon1Spec> {
        Core1LSrstnReqW::new(self, 1)
    }
    #[doc = "Bit 2 - core2_l_srstn request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    #[must_use]
    pub fn core2_l_srstn_req(&mut self) -> Core2LSrstnReqW<SoftrstCon1Spec> {
        Core2LSrstnReqW::new(self, 2)
    }
    #[doc = "Bit 3 - core3_l_srstn request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    #[must_use]
    pub fn core3_l_srstn_req(&mut self) -> Core3LSrstnReqW<SoftrstCon1Spec> {
        Core3LSrstnReqW::new(self, 3)
    }
    #[doc = "Bit 4 - corepo0_l_srstn request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    #[must_use]
    pub fn corepo0_l_srstn_req_t(&mut self) -> Corepo0LSrstnReqTW<SoftrstCon1Spec> {
        Corepo0LSrstnReqTW::new(self, 4)
    }
    #[doc = "Bit 5 - corepo1_l_srstn request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    #[must_use]
    pub fn corepo1_l_srstn_req(&mut self) -> Corepo1LSrstnReqW<SoftrstCon1Spec> {
        Corepo1LSrstnReqW::new(self, 5)
    }
    #[doc = "Bit 6 - corepo2_l_srstn request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    #[must_use]
    pub fn corepo2_l_srstn_req(&mut self) -> Corepo2LSrstnReqW<SoftrstCon1Spec> {
        Corepo2LSrstnReqW::new(self, 6)
    }
    #[doc = "Bit 7 - corepo3_l_srstn request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    #[must_use]
    pub fn corepo3_l_srstn_req(&mut self) -> Corepo3LSrstnReqW<SoftrstCon1Spec> {
        Corepo3LSrstnReqW::new(self, 7)
    }
    #[doc = "Bit 8 - arstn_adb400_gic2corel request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    #[must_use]
    pub fn arstn_adb400_gic2corel_req(&mut self) -> ArstnAdb400Gic2corelReqW<SoftrstCon1Spec> {
        ArstnAdb400Gic2corelReqW::new(self, 8)
    }
    #[doc = "Bit 9 - arstn_adb400_corel2gic request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    #[must_use]
    pub fn arstn_adb400_corel2gic_req(&mut self) -> ArstnAdb400Corel2gicReqW<SoftrstCon1Spec> {
        ArstnAdb400Corel2gicReqW::new(self, 9)
    }
    #[doc = "Bit 10 - prstn_dbg_l request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    #[must_use]
    pub fn prstn_dbg_l_req(&mut self) -> PrstnDbgLReqW<SoftrstCon1Spec> {
        PrstnDbgLReqW::new(self, 10)
    }
    #[doc = "Bit 12 - l2_l_srstn request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    #[must_use]
    pub fn l2_l_srstn_req_t(&mut self) -> L2LSrstnReqTW<SoftrstCon1Spec> {
        L2LSrstnReqTW::new(self, 12)
    }
    #[doc = "Bit 13 - adb_l_srstn request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    #[must_use]
    pub fn adb_l_srstn_req_t(&mut self) -> AdbLSrstnReqTW<SoftrstCon1Spec> {
        AdbLSrstnReqTW::new(self, 13)
    }
    #[doc = "Bit 14 - rkperf_l_arstn request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    #[must_use]
    pub fn rkperf_l_arstn_req(&mut self) -> RkperfLArstnReqW<SoftrstCon1Spec> {
        RkperfLArstnReqW::new(self, 14)
    }
    #[doc = "Bit 15 - pvtm_core_l_srstn request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    #[must_use]
    pub fn pvtm_core_l_srstn_req(&mut self) -> PvtmCoreLSrstnReqW<SoftrstCon1Spec> {
        PvtmCoreLSrstnReqW::new(self, 15)
    }
    #[doc = "Bits 16:31 - write mask bits\n\nWhen every bit HIGH, enable the writing corresponding bit\n\nWhen every bit LOW, don't care the writing corresponding bit"]
    #[inline(always)]
    #[must_use]
    pub fn write_mask(&mut self) -> WriteMaskW<SoftrstCon1Spec> {
        WriteMaskW::new(self, 16)
    }
}
#[doc = "Internal software reset control register1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`softrst_con1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`softrst_con1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SoftrstCon1Spec;
impl crate::RegisterSpec for SoftrstCon1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`softrst_con1::R`](R) reader structure"]
impl crate::Readable for SoftrstCon1Spec {}
#[doc = "`write(|w| ..)` method takes [`softrst_con1::W`](W) writer structure"]
impl crate::Writable for SoftrstCon1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SOFTRST_CON1 to value 0"]
impl crate::Resettable for SoftrstCon1Spec {
    const RESET_VALUE: u32 = 0;
}
