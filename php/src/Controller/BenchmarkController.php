<?php
namespace App\Controller;
use Symfony\Bundle\FrameworkBundle\Controller\AbstractController;
use Symfony\Component\HttpFoundation\JsonResponse;
use Symfony\Component\HttpFoundation\Request;
use Symfony\Component\HttpFoundation\Response;
use Symfony\Component\Routing\Attribute\Route;
class BenchmarkController extends AbstractController {
    #[Route('/', methods: ['GET'])]
    public function hello(): Response {
        return new Response('Hello, world!');
    }
    #[Route('/loop', methods: ['GET'])]
    public function loop(): Response {
        $sum = 0;
        for ($i = 0; $i < 1000000; $i++) {
            $sum += $i;
        }
        return new Response((string)$sum);
    }
    #[Route('/process', methods: ['POST'])]
    public function process(Request $request): JsonResponse {
        $data = json_decode($request->getContent(), true);
        if (!isset($data['client_id'], $data['appointments']) || !is_array($data['appointments'])) {
            return new JsonResponse(['error' => 'Invalid payload'], 400);
        }
        $totalCost = 0.0;
        foreach ($data['appointments'] as $app) {
            $totalCost += ($app['duration_minutes'] / 60.0) * $app['rate'];
        }
        return new JsonResponse([
            'client_id' => $data['client_id'],
            'total_cost' => $totalCost
        ]);
    }
}
