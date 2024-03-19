#[doc = "Register `PROBE_StatGo` reader"]
pub type R = crate::R<ProbeStatGoSpec>;
#[doc = "Register `PROBE_StatGo` writer"]
pub type W = crate::W<ProbeStatGoSpec>;
#[doc = "Field `STATGO` reader - Writing a 1 to the 1-bit pulse register StatGo generates a statistics\n\ndump.The register is active when statistics collection operates in\n\nmanual mode, that is, when register StatPeriod is set to 0.NOTE\n\nThe written value is not stored in StatGo. A read always returns 0."]
pub type StatgoR = crate::BitReader;
#[doc = "Field `STATGO` writer - Writing a 1 to the 1-bit pulse register StatGo generates a statistics\n\ndump.The register is active when statistics collection operates in\n\nmanual mode, that is, when register StatPeriod is set to 0.NOTE\n\nThe written value is not stored in StatGo. A read always returns 0."]
pub type StatgoW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Writing a 1 to the 1-bit pulse register StatGo generates a statistics\n\ndump.The register is active when statistics collection operates in\n\nmanual mode, that is, when register StatPeriod is set to 0.NOTE\n\nThe written value is not stored in StatGo. A read always returns 0."]
    #[inline(always)]
    pub fn statgo(&self) -> StatgoR {
        StatgoR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Writing a 1 to the 1-bit pulse register StatGo generates a statistics\n\ndump.The register is active when statistics collection operates in\n\nmanual mode, that is, when register StatPeriod is set to 0.NOTE\n\nThe written value is not stored in StatGo. A read always returns 0."]
    #[inline(always)]
    #[must_use]
    pub fn statgo(&mut self) -> StatgoW<ProbeStatGoSpec> {
        StatgoW::new(self, 0)
    }
}
#[doc = "Statistics start to dump\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`probe_stat_go::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`probe_stat_go::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ProbeStatGoSpec;
impl crate::RegisterSpec for ProbeStatGoSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`probe_stat_go::R`](R) reader structure"]
impl crate::Readable for ProbeStatGoSpec {}
#[doc = "`write(|w| ..)` method takes [`probe_stat_go::W`](W) writer structure"]
impl crate::Writable for ProbeStatGoSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PROBE_StatGo to value 0"]
impl crate::Resettable for ProbeStatGoSpec {
    const RESET_VALUE: u32 = 0;
}
