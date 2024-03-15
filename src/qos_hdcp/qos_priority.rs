#[doc = "Register `QOS_Priority` reader"]
pub type R = crate::R<QosPrioritySpec>;
#[doc = "Register `QOS_Priority` writer"]
pub type W = crate::W<QosPrioritySpec>;
#[doc = "Field `P0` reader - In Programmable or Bandwidth Limiter mode, the priority level for write transactions. In Bandwidth Regulator mode, the priority level when the used throughput is above the threshold. In Bandwidth Regulator mode, P0 should have a value equal or lower than P1."]
pub type P0R = crate::FieldReader;
#[doc = "Field `P0` writer - In Programmable or Bandwidth Limiter mode, the priority level for write transactions. In Bandwidth Regulator mode, the priority level when the used throughput is above the threshold. In Bandwidth Regulator mode, P0 should have a value equal or lower than P1."]
pub type P0W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `P1` reader - In Programmable or Bandwidth Limiter mode, the priority level for read transactions. In Bandwidth regulator mode, the priority level when the used throughput is below the threshold. In Bandwidth Regulator mode, P1 should have a value equal or greater than P0."]
pub type P1R = crate::FieldReader;
#[doc = "Field `P1` writer - In Programmable or Bandwidth Limiter mode, the priority level for read transactions. In Bandwidth regulator mode, the priority level when the used throughput is below the threshold. In Bandwidth Regulator mode, P1 should have a value equal or greater than P0."]
pub type P1W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `MARK` reader - Backward compatibility marker when 0."]
pub type MarkR = crate::BitReader;
impl R {
    #[doc = "Bits 0:1 - In Programmable or Bandwidth Limiter mode, the priority level for write transactions. In Bandwidth Regulator mode, the priority level when the used throughput is above the threshold. In Bandwidth Regulator mode, P0 should have a value equal or lower than P1."]
    #[inline(always)]
    pub fn p0(&self) -> P0R {
        P0R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - In Programmable or Bandwidth Limiter mode, the priority level for read transactions. In Bandwidth regulator mode, the priority level when the used throughput is below the threshold. In Bandwidth Regulator mode, P1 should have a value equal or greater than P0."]
    #[inline(always)]
    pub fn p1(&self) -> P1R {
        P1R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 31 - Backward compatibility marker when 0."]
    #[inline(always)]
    pub fn mark(&self) -> MarkR {
        MarkR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - In Programmable or Bandwidth Limiter mode, the priority level for write transactions. In Bandwidth Regulator mode, the priority level when the used throughput is above the threshold. In Bandwidth Regulator mode, P0 should have a value equal or lower than P1."]
    #[inline(always)]
    #[must_use]
    pub fn p0(&mut self) -> P0W<QosPrioritySpec> {
        P0W::new(self, 0)
    }
    #[doc = "Bits 2:3 - In Programmable or Bandwidth Limiter mode, the priority level for read transactions. In Bandwidth regulator mode, the priority level when the used throughput is below the threshold. In Bandwidth Regulator mode, P1 should have a value equal or greater than P0."]
    #[inline(always)]
    #[must_use]
    pub fn p1(&mut self) -> P1W<QosPrioritySpec> {
        P1W::new(self, 2)
    }
}
#[doc = "Priority register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`qos_priority::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`qos_priority::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct QosPrioritySpec;
impl crate::RegisterSpec for QosPrioritySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`qos_priority::R`](R) reader structure"]
impl crate::Readable for QosPrioritySpec {}
#[doc = "`write(|w| ..)` method takes [`qos_priority::W`](W) writer structure"]
impl crate::Writable for QosPrioritySpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets QOS_Priority to value 0x8000_0005"]
impl crate::Resettable for QosPrioritySpec {
    const RESET_VALUE: u32 = 0x8000_0005;
}
