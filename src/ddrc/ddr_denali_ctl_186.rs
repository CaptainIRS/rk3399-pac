#[doc = "Register `DDR_DENALI_CTL_186` reader"]
pub type R = crate::R<DdrDenaliCtl186Spec>;
#[doc = "Register `DDR_DENALI_CTL_186` writer"]
pub type W = crate::W<DdrDenaliCtl186Spec>;
#[doc = "Field `TZQCAL_F2` reader - Holds the DRAM ZQCAL value for frequency copy 2 in cycles."]
pub type TzqcalF2R = crate::FieldReader<u16>;
#[doc = "Field `TZQCAL_F2` writer - Holds the DRAM ZQCAL value for frequency copy 2 in cycles."]
pub type TzqcalF2W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `TZQLAT_F2` reader - Holds the DRAM ZQLAT value for frequency copy 2 in cycles."]
pub type TzqlatF2R = crate::FieldReader;
#[doc = "Field `TZQLAT_F2` writer - Holds the DRAM ZQLAT value for frequency copy 2 in cycles."]
pub type TzqlatF2W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `ZQ_SW_REQ_START_LATCH_MAP` reader - Specifies which chip selects will simultaneously receive a ZQ start or latch command once the ZQ_REQ parameter is written with a ZQ Start or ZQ Latch command."]
pub type ZqSwReqStartLatchMapR = crate::FieldReader;
#[doc = "Field `ZQ_SW_REQ_START_LATCH_MAP` writer - Specifies which chip selects will simultaneously receive a ZQ start or latch command once the ZQ_REQ parameter is written with a ZQ Start or ZQ Latch command."]
pub type ZqSwReqStartLatchMapW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:11 - Holds the DRAM ZQCAL value for frequency copy 2 in cycles."]
    #[inline(always)]
    pub fn tzqcal_f2(&self) -> TzqcalF2R {
        TzqcalF2R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:21 - Holds the DRAM ZQLAT value for frequency copy 2 in cycles."]
    #[inline(always)]
    pub fn tzqlat_f2(&self) -> TzqlatF2R {
        TzqlatF2R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 24:25 - Specifies which chip selects will simultaneously receive a ZQ start or latch command once the ZQ_REQ parameter is written with a ZQ Start or ZQ Latch command."]
    #[inline(always)]
    pub fn zq_sw_req_start_latch_map(&self) -> ZqSwReqStartLatchMapR {
        ZqSwReqStartLatchMapR::new(((self.bits >> 24) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:11 - Holds the DRAM ZQCAL value for frequency copy 2 in cycles."]
    #[inline(always)]
    #[must_use]
    pub fn tzqcal_f2(&mut self) -> TzqcalF2W<DdrDenaliCtl186Spec> {
        TzqcalF2W::new(self, 0)
    }
    #[doc = "Bits 16:21 - Holds the DRAM ZQLAT value for frequency copy 2 in cycles."]
    #[inline(always)]
    #[must_use]
    pub fn tzqlat_f2(&mut self) -> TzqlatF2W<DdrDenaliCtl186Spec> {
        TzqlatF2W::new(self, 16)
    }
    #[doc = "Bits 24:25 - Specifies which chip selects will simultaneously receive a ZQ start or latch command once the ZQ_REQ parameter is written with a ZQ Start or ZQ Latch command."]
    #[inline(always)]
    #[must_use]
    pub fn zq_sw_req_start_latch_map(&mut self) -> ZqSwReqStartLatchMapW<DdrDenaliCtl186Spec> {
        ZqSwReqStartLatchMapW::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_ctl_186::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_denali_ctl_186::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliCtl186Spec;
impl crate::RegisterSpec for DdrDenaliCtl186Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_ctl_186::R`](R) reader structure"]
impl crate::Readable for DdrDenaliCtl186Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_denali_ctl_186::W`](W) writer structure"]
impl crate::Writable for DdrDenaliCtl186Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_DENALI_CTL_186 to value 0"]
impl crate::Resettable for DdrDenaliCtl186Spec {
    const RESET_VALUE: u32 = 0;
}
