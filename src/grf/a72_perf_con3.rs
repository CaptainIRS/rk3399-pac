#[doc = "Register `A72_PERF_CON3` reader"]
pub type R = crate::R<A72PerfCon3Spec>;
#[doc = "Register `A72_PERF_CON3` writer"]
pub type W = crate::W<A72PerfCon3Spec>;
#[doc = "Field `MON_ID_MSK` reader - mon_id_msk bit control"]
pub type MonIdMskR = crate::BitReader;
#[doc = "Field `MON_ID_MSK` writer - mon_id_msk bit control"]
pub type MonIdMskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MON_ID_TYPE` reader - mon_id_type bit control"]
pub type MonIdTypeR = crate::BitReader;
#[doc = "Field `MON_ID_TYPE` writer - mon_id_type bit control"]
pub type MonIdTypeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MON_ID_BMSK` reader - mon_id_bmsk bit control"]
pub type MonIdBmskR = crate::FieldReader;
#[doc = "Field `MON_ID_BMSK` writer - mon_id_bmsk bit control"]
pub type MonIdBmskW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `MON_ID` reader - mon_id bit control"]
pub type MonIdR = crate::FieldReader;
#[doc = "Field `MON_ID` writer - mon_id bit control"]
pub type MonIdW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `WRITE_ENABLE` reader - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
pub type WriteEnableR = crate::FieldReader<u16>;
#[doc = "Field `WRITE_ENABLE` writer - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
pub type WriteEnableW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bit 0 - mon_id_msk bit control"]
    #[inline(always)]
    pub fn mon_id_msk(&self) -> MonIdMskR {
        MonIdMskR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - mon_id_type bit control"]
    #[inline(always)]
    pub fn mon_id_type(&self) -> MonIdTypeR {
        MonIdTypeR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:8 - mon_id_bmsk bit control"]
    #[inline(always)]
    pub fn mon_id_bmsk(&self) -> MonIdBmskR {
        MonIdBmskR::new(((self.bits >> 2) & 0x7f) as u8)
    }
    #[doc = "Bits 9:15 - mon_id bit control"]
    #[inline(always)]
    pub fn mon_id(&self) -> MonIdR {
        MonIdR::new(((self.bits >> 9) & 0x7f) as u8)
    }
    #[doc = "Bits 16:31 - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
    #[inline(always)]
    pub fn write_enable(&self) -> WriteEnableR {
        WriteEnableR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - mon_id_msk bit control"]
    #[inline(always)]
    #[must_use]
    pub fn mon_id_msk(&mut self) -> MonIdMskW<A72PerfCon3Spec> {
        MonIdMskW::new(self, 0)
    }
    #[doc = "Bit 1 - mon_id_type bit control"]
    #[inline(always)]
    #[must_use]
    pub fn mon_id_type(&mut self) -> MonIdTypeW<A72PerfCon3Spec> {
        MonIdTypeW::new(self, 1)
    }
    #[doc = "Bits 2:8 - mon_id_bmsk bit control"]
    #[inline(always)]
    #[must_use]
    pub fn mon_id_bmsk(&mut self) -> MonIdBmskW<A72PerfCon3Spec> {
        MonIdBmskW::new(self, 2)
    }
    #[doc = "Bits 9:15 - mon_id bit control"]
    #[inline(always)]
    #[must_use]
    pub fn mon_id(&mut self) -> MonIdW<A72PerfCon3Spec> {
        MonIdW::new(self, 9)
    }
    #[doc = "Bits 16:31 - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
    #[inline(always)]
    #[must_use]
    pub fn write_enable(&mut self) -> WriteEnableW<A72PerfCon3Spec> {
        WriteEnableW::new(self, 16)
    }
}
#[doc = "a72 performance monitor control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`a72_perf_con3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`a72_perf_con3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct A72PerfCon3Spec;
impl crate::RegisterSpec for A72PerfCon3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`a72_perf_con3::R`](R) reader structure"]
impl crate::Readable for A72PerfCon3Spec {}
#[doc = "`write(|w| ..)` method takes [`a72_perf_con3::W`](W) writer structure"]
impl crate::Writable for A72PerfCon3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets A72_PERF_CON3 to value 0"]
impl crate::Resettable for A72PerfCon3Spec {
    const RESET_VALUE: u32 = 0;
}
